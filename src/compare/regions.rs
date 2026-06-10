//! Localized error regions.
//!
//! The image is divided into a grid of blocks; each block's mean RGB error is
//! measured, contiguous high-error blocks are merged into bounding boxes, and
//! the boxes are ranked by total error so an agent can see *where* its SVG is
//! wrong, not just by how much.

use super::luma::Prepared;
use serde::Serialize;

/// A rectangular area of disagreement, in pixel coordinates of the reference.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    /// Mean per-channel error inside the box, `0.0..=100.0` (higher = worse).
    pub error: f64,
}

/// Tuning for region detection.
#[derive(Debug, Clone)]
pub struct RegionConfig {
    /// Block edge length in pixels. Smaller = finer localization, more regions.
    pub block: u32,
    /// Maximum number of regions to return (ranked by total error).
    pub max_regions: usize,
    /// A block is "hot" only if its mean error meets this floor (0–100).
    pub min_error: f64,
}

impl RegionConfig {
    /// Pick a block size that yields roughly a 16-cell grid across the longest
    /// side, clamped to a sane pixel range.
    pub fn auto(width: u32, height: u32) -> Self {
        let longest = width.max(height).max(1);
        let block = (longest / 16).clamp(8, 64);
        Self {
            block,
            max_regions: 8,
            min_error: 2.0,
        }
    }
}

/// Compute ranked error regions between two prepared images of equal size.
pub fn regions(a: &Prepared, b: &Prepared, cfg: &RegionConfig) -> Vec<Region> {
    let (w, h) = (a.width, a.height);
    if w == 0 || h == 0 {
        return Vec::new();
    }
    let block = cfg.block.max(1);
    let cols = w.div_ceil(block) as usize;
    let rows = h.div_ceil(block) as usize;

    // Mean error per block, 0..100.
    let mut grid = vec![0f64; cols * rows];
    for (gy, row) in grid.chunks_mut(cols).enumerate() {
        for (gx, cell) in row.iter_mut().enumerate() {
            *cell = block_error(a, b, gx as u32 * block, gy as u32 * block, block);
        }
    }

    // Flood-fill connected hot blocks (4-connectivity) into components.
    let mut visited = vec![false; cols * rows];
    let mut out = Vec::new();
    for start in 0..(cols * rows) {
        if visited[start] || grid[start] < cfg.min_error {
            continue;
        }
        let component = flood(start, cols, rows, &grid, cfg.min_error, &mut visited);
        out.push(component.into_region(block, w, h));
    }

    // Rank by total error contribution (mean error * area), descending.
    out.sort_by(|p, q| {
        let pp = p.error * (p.width as f64 * p.height as f64);
        let qq = q.error * (q.width as f64 * q.height as f64);
        qq.partial_cmp(&pp).unwrap_or(std::cmp::Ordering::Equal)
    });
    out.truncate(cfg.max_regions);
    out
}

/// Mean per-channel error over the block at `(ox, oy)`, clipped to the image.
fn block_error(a: &Prepared, b: &Prepared, ox: u32, oy: u32, block: u32) -> f64 {
    let w = a.width;
    let h = a.height;
    let x1 = (ox + block).min(w);
    let y1 = (oy + block).min(h);
    let mut sum = 0u64;
    let mut n = 0u64;
    for y in oy..y1 {
        let row = (y * w) as usize * 3;
        for x in ox..x1 {
            let i = row + x as usize * 3;
            sum += a.rgb[i].abs_diff(b.rgb[i]) as u64
                + a.rgb[i + 1].abs_diff(b.rgb[i + 1]) as u64
                + a.rgb[i + 2].abs_diff(b.rgb[i + 2]) as u64;
            n += 3;
        }
    }
    if n == 0 {
        0.0
    } else {
        (sum as f64 / n as f64) / 255.0 * 100.0
    }
}

/// An accumulating connected component of hot blocks.
struct Component {
    gx0: usize,
    gy0: usize,
    gx1: usize,
    gy1: usize,
    error_sum: f64,
    blocks: f64,
}

impl Component {
    fn into_region(self, block: u32, w: u32, h: u32) -> Region {
        let x = self.gx0 as u32 * block;
        let y = self.gy0 as u32 * block;
        let x1 = ((self.gx1 as u32 + 1) * block).min(w);
        let y1 = ((self.gy1 as u32 + 1) * block).min(h);
        Region {
            x,
            y,
            width: x1 - x,
            height: y1 - y,
            error: self.error_sum / self.blocks.max(1.0),
        }
    }
}

/// Iterative flood fill collecting one connected component of hot blocks.
fn flood(
    start: usize,
    cols: usize,
    rows: usize,
    grid: &[f64],
    min_error: f64,
    visited: &mut [bool],
) -> Component {
    let mut stack = vec![start];
    visited[start] = true;
    let (sx, sy) = (start % cols, start / cols);
    let mut comp = Component {
        gx0: sx,
        gy0: sy,
        gx1: sx,
        gy1: sy,
        error_sum: 0.0,
        blocks: 0.0,
    };
    while let Some(idx) = stack.pop() {
        let (gx, gy) = (idx % cols, idx / cols);
        comp.gx0 = comp.gx0.min(gx);
        comp.gy0 = comp.gy0.min(gy);
        comp.gx1 = comp.gx1.max(gx);
        comp.gy1 = comp.gy1.max(gy);
        comp.error_sum += grid[idx];
        comp.blocks += 1.0;
        let mut neighbors = [(0isize, 0isize); 4];
        neighbors[0] = (1, 0);
        neighbors[1] = (-1, 0);
        neighbors[2] = (0, 1);
        neighbors[3] = (0, -1);
        for (dx, dy) in neighbors {
            let nx = gx as isize + dx;
            let ny = gy as isize + dy;
            if nx < 0 || ny < 0 || nx >= cols as isize || ny >= rows as isize {
                continue;
            }
            let nidx = ny as usize * cols + nx as usize;
            if !visited[nidx] && grid[nidx] >= min_error {
                visited[nidx] = true;
                stack.push(nidx);
            }
        }
    }
    comp
}
