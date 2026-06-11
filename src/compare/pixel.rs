//! Pixel-level similarity.

use super::luma::Prepared;

/// Mean per-channel agreement between two white-composited RGB buffers,
/// expressed as a percentage in `0.0..=100.0`.
///
/// Defined as `100 * (1 - MAE/255)` where MAE is the mean absolute difference
/// across all R, G, B channels. Identical images score 100; an image and its
/// photographic negative score 0.
pub fn pixel_match(a: &Prepared, b: &Prepared) -> f64 {
    debug_assert_eq!(a.rgb.len(), b.rgb.len());
    if a.rgb.is_empty() {
        return 100.0;
    }
    let mut sum = 0u64;
    for (x, y) in a.rgb.iter().zip(b.rgb.iter()) {
        sum += x.abs_diff(*y) as u64;
    }
    let mae = sum as f64 / a.rgb.len() as f64;
    (1.0 - mae / 255.0) * 100.0
}
