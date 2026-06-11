//! Pixel preparation shared by every comparison metric.
//!
//! SVG output is routinely partially transparent, so before comparing we
//! composite both images over an opaque white background. This makes the
//! comparison deterministic and matches how a human (or a vision model) would
//! see the rendered result on a typical page.

use image::RgbaImage;

/// White-composited RGB and luma views of an image, computed once and reused
/// by the pixel, SSIM, and region metrics.
pub struct Prepared {
    pub width: u32,
    pub height: u32,
    /// Straight RGB, row-major, 3 bytes per pixel, composited over white.
    pub rgb: Vec<u8>,
    /// Rec. 601 luma in `0.0..=255.0`, one value per pixel.
    pub luma: Vec<f32>,
}

/// Composite `img` over opaque white and precompute RGB + luma buffers.
pub fn prepare(img: &RgbaImage) -> Prepared {
    let (w, h) = img.dimensions();
    let count = (w * h) as usize;
    let mut rgb = vec![0u8; count * 3];
    let mut luma = vec![0f32; count];
    for (i, px) in img.pixels().enumerate() {
        let alpha = px[3] as f32 / 255.0;
        let r = blend(px[0], alpha);
        let g = blend(px[1], alpha);
        let b = blend(px[2], alpha);
        rgb[i * 3] = r;
        rgb[i * 3 + 1] = g;
        rgb[i * 3 + 2] = b;
        luma[i] = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
    }
    Prepared {
        width: w,
        height: h,
        rgb,
        luma,
    }
}

/// Composite a single channel over a white background.
#[inline]
fn blend(channel: u8, alpha: f32) -> u8 {
    let fg = channel as f32 * alpha;
    let bg = 255.0 * (1.0 - alpha);
    (fg + bg).round().clamp(0.0, 255.0) as u8
}
