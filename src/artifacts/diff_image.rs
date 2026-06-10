//! Diff heatmap: per-pixel error painted over a ghost of the reference.

use crate::artifacts::compose::flatten_over_white;
use anyhow::{ensure, Result};
use image::{Rgba, RgbaImage};

/// Render a heatmap where per-pixel disagreement is shown as a red overlay on a
/// faded grayscale reference, so an agent sees both *where* and *how much* its
/// candidate diverges. Inputs must share dimensions.
pub fn diff_image(reference: &RgbaImage, candidate: &RgbaImage) -> Result<RgbaImage> {
    ensure!(
        reference.dimensions() == candidate.dimensions(),
        "diff image requires equal dimensions"
    );
    let r = flatten_over_white(reference);
    let c = flatten_over_white(candidate);
    let mut out = RgbaImage::new(reference.width(), reference.height());

    for ((rp, cp), op) in r.pixels().zip(c.pixels()).zip(out.pixels_mut()) {
        // Faded grayscale base from the reference for context.
        let luma = 0.299 * rp[0] as f32 + 0.587 * rp[1] as f32 + 0.114 * rp[2] as f32;
        let base = 200.0 + (luma - 200.0) * 0.25; // compress toward light gray
        let base = base.clamp(160.0, 245.0) as u8;

        // Error fraction across RGB.
        let err = (rp[0].abs_diff(cp[0]) as f32
            + rp[1].abs_diff(cp[1]) as f32
            + rp[2].abs_diff(cp[2]) as f32)
            / (3.0 * 255.0);

        // Blend a hot color in proportion to the error.
        let (hr, hg, hb) = heat(err);
        let a = err; // overlay strength tracks error
        let mix = |b: u8, h: u8| ((b as f32) * (1.0 - a) + h as f32 * a).round() as u8;
        *op = Rgba([mix(base, hr), mix(base, hg), mix(base, hb), 255]);
    }
    Ok(out)
}

/// Map an error fraction `0.0..=1.0` to a yellow→red hot color.
fn heat(err: f32) -> (u8, u8, u8) {
    let e = err.clamp(0.0, 1.0);
    // 0 -> yellow (255,220,0), 1 -> deep red (200,0,0)
    let r = 255.0 + (200.0 - 255.0) * e;
    let g = 220.0 + (0.0 - 220.0) * e;
    let b = 0.0;
    (r as u8, g as u8, b as u8)
}
