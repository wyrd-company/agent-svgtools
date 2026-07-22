//! Pixel-level similarity.

use super::luma::Prepared;

/// Mean per-channel agreement between two white-composited RGB buffers,
/// expressed as a percentage in `0.0..=100.0`.
///
/// Defined as `100 * (1 - MAE/255)` where MAE is the mean absolute difference
/// across all R, G, B channels. Identical images score 100; an image and its
/// photographic negative score 0.
pub fn pixel_match(a: &Prepared, b: &Prepared) -> f64 {
    pixel_match_masked(a, b, None)
}

/// Pixel agreement restricted to foreground pixels and multiplied by alpha
/// agreement so adding an opaque background is penalized without diluting RGB
/// disagreement when both inputs have the same silhouette.
pub fn pixel_match_foreground(a: &Prepared, b: &Prepared, mask: &[bool]) -> f64 {
    let rgb = pixel_match_masked(a, b, Some(mask));
    let mut alpha_difference = 0u64;
    let mut pixels = 0usize;
    for (i, (x, y)) in a.alpha.iter().zip(&b.alpha).enumerate() {
        if !mask[i] {
            continue;
        }
        alpha_difference += x.abs_diff(*y) as u64;
        pixels += 1;
    }
    if pixels == 0 {
        return 100.0;
    }
    let alpha = (1.0 - alpha_difference as f64 / pixels as f64 / 255.0) * 100.0;
    rgb * alpha / 100.0
}

fn pixel_match_masked(a: &Prepared, b: &Prepared, mask: Option<&[bool]>) -> f64 {
    debug_assert_eq!(a.rgb.len(), b.rgb.len());
    if a.rgb.is_empty() {
        return 100.0;
    }
    let mut sum = 0u64;
    let mut channels = 0usize;
    for (i, (a_px, b_px)) in a.rgb.chunks_exact(3).zip(b.rgb.chunks_exact(3)).enumerate() {
        if mask.is_some_and(|mask| !mask[i]) {
            continue;
        }
        for (x, y) in a_px.iter().zip(b_px.iter()) {
            sum += x.abs_diff(*y) as u64;
            channels += 1;
        }
    }
    if channels == 0 {
        return 100.0;
    }
    let mae = sum as f64 / channels as f64;
    (1.0 - mae / 255.0) * 100.0
}
