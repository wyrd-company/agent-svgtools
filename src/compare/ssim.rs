//! Structural Similarity (SSIM).
//!
//! A from-scratch implementation of the Wang et al. (2004) mean-SSIM index over
//! a Gaussian window. Implemented in-tree (rather than pulling a GPL/AGPL crate)
//! so the project stays MIT-licensed and the scores are deterministic and unit
//! testable.
//!
//! Local statistics are gathered with a separable 11-tap Gaussian (sigma 1.5),
//! giving the standard windowed behavior at O(n) per pass instead of O(n*k^2).

use super::luma::Prepared;

const K1: f64 = 0.01;
const K2: f64 = 0.03;
const L: f64 = 255.0; // dynamic range of luma
const GAUSS_SIGMA: f32 = 1.5;
const GAUSS_RADIUS: usize = 5; // 11-tap window

/// Mean SSIM between two luma planes, scaled to `0.0..=100.0`.
///
/// The raw index lies in `-1.0..=1.0`; we clamp to `0.0..=1.0` and scale so the
/// score shares a 0–100 axis with [`super::pixel::pixel_match`].
pub fn ssim(a: &Prepared, b: &Prepared) -> f64 {
    ssim_plane(&a.luma, &b.luma, a.width as usize, a.height as usize, None)
}

/// Mean foreground SSIM over composited luma and original alpha. Including the
/// alpha plane prevents an opaque background from masquerading as matching
/// white canvas.
pub fn ssim_foreground(a: &Prepared, b: &Prepared, mask: &[bool]) -> f64 {
    let alpha_a = a
        .alpha
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>();
    let alpha_b = b
        .alpha
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>();
    let luma = ssim_plane(
        &a.luma,
        &b.luma,
        a.width as usize,
        a.height as usize,
        Some(mask),
    );
    let alpha = ssim_plane(
        &alpha_a,
        &alpha_b,
        a.width as usize,
        a.height as usize,
        Some(mask),
    );
    luma * alpha / 100.0
}

/// Mean SSIM restricted to centers selected by `mask`. Local statistics still
/// use the normal Gaussian neighborhood so silhouette-edge structure remains
/// meaningful.
fn ssim_plane(x: &[f32], y: &[f32], w: usize, h: usize, mask: Option<&[bool]>) -> f64 {
    if w == 0 || h == 0 {
        return 100.0;
    }
    let kernel = gaussian_kernel(GAUSS_RADIUS, GAUSS_SIGMA);

    let xx: Vec<f32> = x.iter().map(|v| v * v).collect();
    let yy: Vec<f32> = y.iter().map(|v| v * v).collect();
    let xy: Vec<f32> = x.iter().zip(y).map(|(p, q)| p * q).collect();

    let mu_x = blur_separable(x, w, h, &kernel);
    let mu_y = blur_separable(y, w, h, &kernel);
    let mu_xx = blur_separable(&xx, w, h, &kernel);
    let mu_yy = blur_separable(&yy, w, h, &kernel);
    let mu_xy = blur_separable(&xy, w, h, &kernel);

    let c1 = (K1 * L).powi(2);
    let c2 = (K2 * L).powi(2);

    let mut total = 0.0f64;
    let mut count = 0usize;
    for i in 0..(w * h) {
        if mask.is_some_and(|mask| !mask[i]) {
            continue;
        }
        let ux = mu_x[i] as f64;
        let uy = mu_y[i] as f64;
        let vx = (mu_xx[i] as f64 - ux * ux).max(0.0);
        let vy = (mu_yy[i] as f64 - uy * uy).max(0.0);
        let vxy = mu_xy[i] as f64 - ux * uy;
        let numerator = (2.0 * ux * uy + c1) * (2.0 * vxy + c2);
        let denominator = (ux * ux + uy * uy + c1) * (vx + vy + c2);
        total += numerator / denominator;
        count += 1;
    }
    if count == 0 {
        return 100.0;
    }
    let mean = total / count as f64;
    mean.clamp(0.0, 1.0) * 100.0
}

/// Build a normalized 1-D Gaussian kernel of `2*radius + 1` taps.
fn gaussian_kernel(radius: usize, sigma: f32) -> Vec<f32> {
    let mut k = Vec::with_capacity(2 * radius + 1);
    let two_sigma_sq = 2.0 * sigma * sigma;
    for i in 0..=(2 * radius) {
        let x = i as f32 - radius as f32;
        k.push((-(x * x) / two_sigma_sq).exp());
    }
    let sum: f32 = k.iter().sum();
    for v in &mut k {
        *v /= sum;
    }
    k
}

/// Separable Gaussian blur with clamped (edge-replicated) borders.
fn blur_separable(src: &[f32], w: usize, h: usize, kernel: &[f32]) -> Vec<f32> {
    let radius = kernel.len() / 2;
    let mut tmp = vec![0f32; w * h];
    // Horizontal pass.
    for y in 0..h {
        let row = y * w;
        for x in 0..w {
            let mut acc = 0.0;
            for (k, &weight) in kernel.iter().enumerate() {
                let sx = clamp_index(x as isize + k as isize - radius as isize, w);
                acc += src[row + sx] * weight;
            }
            tmp[row + x] = acc;
        }
    }
    // Vertical pass.
    let mut out = vec![0f32; w * h];
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0.0;
            for (k, &weight) in kernel.iter().enumerate() {
                let sy = clamp_index(y as isize + k as isize - radius as isize, h);
                acc += tmp[sy * w + x] * weight;
            }
            out[y * w + x] = acc;
        }
    }
    out
}

#[inline]
fn clamp_index(i: isize, len: usize) -> usize {
    i.clamp(0, len as isize - 1) as usize
}
