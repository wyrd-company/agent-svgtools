//! Low-level pixel composition helpers shared by the artifact generators.

use image::{Rgba, RgbaImage};

/// Composite `img` over an opaque white background, returning an opaque image.
pub fn flatten_over_white(img: &RgbaImage) -> RgbaImage {
    let mut out = RgbaImage::new(img.width(), img.height());
    for (o, p) in out.pixels_mut().zip(img.pixels()) {
        let a = p[3] as f32 / 255.0;
        *o = Rgba([
            over_white(p[0], a),
            over_white(p[1], a),
            over_white(p[2], a),
            255,
        ]);
    }
    out
}

#[inline]
fn over_white(c: u8, a: f32) -> u8 {
    (c as f32 * a + 255.0 * (1.0 - a)).round().clamp(0.0, 255.0) as u8
}

/// Source-over composite `top` onto `base` at `(ox, oy)`, in place.
///
/// Both images use straight (non-premultiplied) alpha. Pixels of `top` that
/// fall outside `base` are clipped.
pub fn over(base: &mut RgbaImage, top: &RgbaImage, ox: i64, oy: i64) {
    let (bw, bh) = base.dimensions();
    for ty in 0..top.height() {
        let y = oy + ty as i64;
        if y < 0 || y >= bh as i64 {
            continue;
        }
        for tx in 0..top.width() {
            let x = ox + tx as i64;
            if x < 0 || x >= bw as i64 {
                continue;
            }
            let src = *top.get_pixel(tx, ty);
            if src[3] == 0 {
                continue;
            }
            let dst = base.get_pixel_mut(x as u32, y as u32);
            *dst = blend(*dst, src);
        }
    }
}

/// Straight-alpha source-over of `src` onto `dst`.
fn blend(dst: Rgba<u8>, src: Rgba<u8>) -> Rgba<u8> {
    let sa = src[3] as f32 / 255.0;
    let da = dst[3] as f32 / 255.0;
    let out_a = sa + da * (1.0 - sa);
    if out_a <= 0.0 {
        return Rgba([0, 0, 0, 0]);
    }
    let mix = |s: u8, d: u8| -> u8 {
        let v = (s as f32 * sa + d as f32 * da * (1.0 - sa)) / out_a;
        v.round().clamp(0.0, 255.0) as u8
    };
    Rgba([
        mix(src[0], dst[0]),
        mix(src[1], dst[1]),
        mix(src[2], dst[2]),
        (out_a * 255.0).round() as u8,
    ])
}
