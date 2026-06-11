//! Dominant-color palette extraction.
//!
//! Logos are the primary input, so the goal is to recover the handful of flat
//! colors an agent should reach for. The method is intentionally simple and
//! deterministic: bucket pixels in a coarse RGB histogram to seed representative
//! colors, greedily de-duplicate near-identical seeds, then assign every pixel
//! to its nearest representative to get accurate coverage and a sample location.

use crate::util::color;
use image::RgbaImage;
use serde::Serialize;

/// One entry in an extracted palette.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PaletteColor {
    /// `#rrggbb` for direct use in SVG `fill`/`stroke`.
    pub hex: String,
    pub rgb: [u8; 3],
    /// Share of the image assigned to this color, 0–100.
    pub coverage: f64,
    /// A pixel that exemplifies this color, for cross-referencing the image.
    pub location: Location,
}

/// A pixel coordinate.
#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

/// Extract up to `max_colors` dominant colors, ordered by coverage descending.
pub fn extract(img: &RgbaImage, max_colors: usize) -> Vec<PaletteColor> {
    let max_colors = max_colors.max(1);
    let (w, h) = img.dimensions();
    if w == 0 || h == 0 {
        return Vec::new();
    }

    // Composite over white so transparency maps to a concrete color.
    let rgb: Vec<[u8; 3]> = img
        .pixels()
        .map(|p| {
            let a = p[3] as f32 / 255.0;
            [comp(p[0], a), comp(p[1], a), comp(p[2], a)]
        })
        .collect();

    let reps = seed_representatives(&rgb, max_colors);
    assign_and_summarize(&rgb, w, &reps)
}

/// Composite one channel over white.
#[inline]
fn comp(c: u8, a: f32) -> u8 {
    (c as f32 * a + 255.0 * (1.0 - a)).round().clamp(0.0, 255.0) as u8
}

/// Seed representative colors from the densest coarse histogram bins, skipping
/// seeds that are within `MIN_SEED_DIST` of an already-chosen one.
fn seed_representatives(rgb: &[[u8; 3]], max_colors: usize) -> Vec<[u8; 3]> {
    const BITS: u8 = 5; // 32 levels per channel
    const SHIFT: u8 = 8 - BITS;
    const MIN_SEED_DIST: i64 = 40 * 40; // squared euclidean

    let mut counts: std::collections::HashMap<u16, (u64, [u64; 3])> =
        std::collections::HashMap::new();
    for px in rgb {
        let key = ((px[0] >> SHIFT) as u16) << (2 * BITS)
            | ((px[1] >> SHIFT) as u16) << BITS
            | (px[2] >> SHIFT) as u16;
        let entry = counts.entry(key).or_insert((0, [0, 0, 0]));
        entry.0 += 1;
        entry.1[0] += px[0] as u64;
        entry.1[1] += px[1] as u64;
        entry.1[2] += px[2] as u64;
    }

    // Mean color per bin, sorted by population (then color for stability).
    let mut bins: Vec<(u64, [u8; 3])> = counts
        .into_values()
        .map(|(n, sum)| {
            (
                n,
                [(sum[0] / n) as u8, (sum[1] / n) as u8, (sum[2] / n) as u8],
            )
        })
        .collect();
    bins.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

    let mut reps: Vec<[u8; 3]> = Vec::new();
    for (_, color) in bins {
        if reps.len() >= max_colors {
            break;
        }
        if reps.iter().all(|r| dist2(*r, color) >= MIN_SEED_DIST) {
            reps.push(color);
        }
    }
    reps
}

/// Assign every pixel to its nearest representative, producing coverage and a
/// representative location (the pixel closest to each representative color).
fn assign_and_summarize(rgb: &[[u8; 3]], w: u32, reps: &[[u8; 3]]) -> Vec<PaletteColor> {
    let total = rgb.len() as f64;
    let mut counts = vec![0u64; reps.len()];
    // Track the best (closest) sample location per representative.
    let mut best: Vec<(i64, u32)> = vec![(i64::MAX, 0); reps.len()];

    for (i, px) in rgb.iter().enumerate() {
        let mut best_rep = 0usize;
        let mut best_d = i64::MAX;
        for (r, rep) in reps.iter().enumerate() {
            let d = dist2(*px, *rep);
            if d < best_d {
                best_d = d;
                best_rep = r;
            }
        }
        counts[best_rep] += 1;
        if best_d < best[best_rep].0 {
            best[best_rep] = (best_d, i as u32);
        }
    }

    let mut out: Vec<PaletteColor> = reps
        .iter()
        .enumerate()
        .filter(|(r, _)| counts[*r] > 0)
        .map(|(r, rep)| {
            let idx = best[r].1;
            PaletteColor {
                hex: color::to_hex(image::Rgba([rep[0], rep[1], rep[2], 255])),
                rgb: *rep,
                coverage: round2(counts[r] as f64 / total * 100.0),
                location: Location {
                    x: idx % w,
                    y: idx / w,
                },
            }
        })
        .collect();
    out.sort_by(|a, b| {
        b.coverage
            .partial_cmp(&a.coverage)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    out
}

#[inline]
fn dist2(a: [u8; 3], b: [u8; 3]) -> i64 {
    let dr = a[0] as i64 - b[0] as i64;
    let dg = a[1] as i64 - b[1] as i64;
    let db = a[2] as i64 - b[2] as i64;
    dr * dr + dg * dg + db * db
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    #[test]
    fn extracts_two_dominant_colors() {
        let mut img = RgbaImage::from_pixel(20, 10, Rgba([255, 0, 0, 255]));
        for y in 0..10 {
            for x in 10..20 {
                img.put_pixel(x, y, Rgba([0, 0, 255, 255]));
            }
        }
        let palette = extract(&img, 4);
        assert_eq!(palette.len(), 2);
        // Equal halves, ~50% each.
        assert!((palette[0].coverage - 50.0).abs() < 1.0);
        let hexes: Vec<&str> = palette.iter().map(|c| c.hex.as_str()).collect();
        assert!(hexes.contains(&"#ff0000"));
        assert!(hexes.contains(&"#0000ff"));
    }

    #[test]
    fn respects_max_colors() {
        let mut img = RgbaImage::new(8, 8);
        for (i, px) in img.pixels_mut().enumerate() {
            *px = Rgba([(i * 4) as u8, 0, 0, 255]);
        }
        assert!(extract(&img, 2).len() <= 2);
    }
}
