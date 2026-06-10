//! Onion-skin blend: reference and candidate at 50% opacity each.

use crate::artifacts::compose::flatten_over_white;
use anyhow::{ensure, Result};
use image::{Rgba, RgbaImage};

/// Produce a 50/50 onion-skin overlay. Where the two images agree the result
/// is crisp; where they diverge it ghosts, making misalignment obvious.
/// Inputs must share dimensions.
pub fn blend(reference: &RgbaImage, candidate: &RgbaImage) -> Result<RgbaImage> {
    ensure!(
        reference.dimensions() == candidate.dimensions(),
        "blend requires equal dimensions"
    );
    let r = flatten_over_white(reference);
    let c = flatten_over_white(candidate);
    let mut out = RgbaImage::new(reference.width(), reference.height());
    for ((rp, cp), op) in r.pixels().zip(c.pixels()).zip(out.pixels_mut()) {
        let avg = |a: u8, b: u8| ((a as u16 + b as u16) / 2) as u8;
        *op = Rgba([avg(rp[0], cp[0]), avg(rp[1], cp[1]), avg(rp[2], cp[2]), 255]);
    }
    Ok(out)
}
