//! The scoring engine: pixel match, SSIM, and localized error regions.
//!
//! The engine is pure and dimension-strict: it compares two equally-sized RGBA
//! buffers. Resizing and dimension/aspect warnings are the caller's job (see
//! [`crate::commands`]), keeping the metrics deterministic and unit-testable.

mod luma;
mod pixel;
mod regions;
mod ssim;

pub use regions::{Region, RegionConfig};

use anyhow::{ensure, Result};
use image::RgbaImage;
use serde::Serialize;

/// Relative weight of pixel match in the combined score (SSIM gets the rest).
const PIXEL_WEIGHT: f64 = 0.5;

/// The full result of comparing a reference and a candidate image.
#[derive(Debug, Clone, Serialize)]
pub struct Comparison {
    pub width: u32,
    pub height: u32,
    /// Per-channel agreement, 0–100.
    pub pixel_match: f64,
    /// Structural similarity, 0–100.
    pub ssim: f64,
    /// Weighted blend of the two, 0–100. The single number to threshold on.
    pub combined: f64,
    /// High-error areas, ranked worst-first.
    pub regions: Vec<Region>,
}

/// Compare two equally-sized images, computing all metrics and regions.
pub fn compare(a: &RgbaImage, b: &RgbaImage, region_cfg: &RegionConfig) -> Result<Comparison> {
    ensure!(
        a.dimensions() == b.dimensions(),
        "comparison requires equal dimensions: {:?} vs {:?}",
        a.dimensions(),
        b.dimensions()
    );
    let pa = luma::prepare(a);
    let pb = luma::prepare(b);

    let pixel_match = pixel::pixel_match(&pa, &pb);
    let ssim = ssim::ssim(&pa, &pb);
    let combined = PIXEL_WEIGHT * pixel_match + (1.0 - PIXEL_WEIGHT) * ssim;
    let regions = regions::regions(&pa, &pb, region_cfg);

    Ok(Comparison {
        width: pa.width,
        height: pa.height,
        pixel_match: round2(pixel_match),
        ssim: round2(ssim),
        combined: round2(combined),
        regions: regions
            .into_iter()
            .map(|mut r| {
                r.error = round2(r.error);
                r
            })
            .collect(),
    })
}

/// Round to two decimals so output is stable and snapshot-friendly.
fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    fn solid(w: u32, h: u32, color: [u8; 4]) -> RgbaImage {
        RgbaImage::from_pixel(w, h, Rgba(color))
    }

    #[test]
    fn identical_images_score_perfectly() {
        let img = solid(32, 32, [120, 30, 200, 255]);
        let cfg = RegionConfig::auto(32, 32);
        let c = compare(&img, &img, &cfg).unwrap();
        assert_eq!(c.pixel_match, 100.0);
        assert!(c.ssim > 99.9, "ssim was {}", c.ssim);
        assert_eq!(c.combined, 100.0);
        assert!(c.regions.is_empty());
    }

    #[test]
    fn black_vs_white_scores_zero_pixels() {
        let black = solid(16, 16, [0, 0, 0, 255]);
        let white = solid(16, 16, [255, 255, 255, 255]);
        let cfg = RegionConfig::auto(16, 16);
        let c = compare(&black, &white, &cfg).unwrap();
        assert_eq!(c.pixel_match, 0.0);
        // Whole frame differs, so a single region covers it.
        assert_eq!(c.regions.len(), 1);
        let r = &c.regions[0];
        assert_eq!((r.x, r.y, r.width, r.height), (0, 0, 16, 16));
        assert!(r.error > 99.0);
    }

    #[test]
    fn localized_difference_is_boxed() {
        let mut a = solid(64, 64, [255, 255, 255, 255]);
        let mut b = a.clone();
        // Paint a divergent 16x16 patch only in `b`.
        for y in 8..24 {
            for x in 8..24 {
                a.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                b.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            }
        }
        let cfg = RegionConfig {
            block: 8,
            max_regions: 8,
            min_error: 2.0,
        };
        let c = compare(&a, &b, &cfg).unwrap();
        assert_eq!(c.regions.len(), 1);
        let r = &c.regions[0];
        assert!(r.x <= 8 && r.y <= 8);
        assert!(r.x + r.width >= 24 && r.y + r.height >= 24);
        assert!(c.pixel_match > 90.0 && c.pixel_match < 100.0);
    }

    #[test]
    fn rejects_mismatched_dimensions() {
        let a = solid(10, 10, [0, 0, 0, 255]);
        let b = solid(12, 10, [0, 0, 0, 255]);
        let cfg = RegionConfig::auto(10, 10);
        assert!(compare(&a, &b, &cfg).is_err());
    }
}
