//! SVG rasterization onto an RGBA pixel buffer.
//!
//! Wraps `usvg` (parsing) and `resvg`/`tiny-skia` (rasterization) behind a
//! small, intent-revealing API. The renderer never touches the filesystem; it
//! takes SVG bytes and returns an [`image::RgbaImage`].

use anyhow::{Context, Result};
use image::{Rgba, RgbaImage};
use resvg::tiny_skia;
use resvg::usvg;

/// How large to rasterize, and on what background.
#[derive(Debug, Clone, Default)]
pub struct RenderOptions {
    /// Explicit target width in pixels.
    pub width: Option<u32>,
    /// Explicit target height in pixels.
    pub height: Option<u32>,
    /// Uniform scale applied to the SVG's intrinsic size. Ignored when an
    /// explicit width or height is supplied.
    pub scale: Option<f32>,
    /// Background fill. `None` leaves transparent pixels transparent.
    pub background: Option<Rgba<u8>>,
}

impl RenderOptions {
    /// Render at exactly `width`x`height`, stretching the SVG to fill. This is
    /// what `judge`/`compare` use so the candidate aligns with the reference
    /// canvas pixel-for-pixel.
    pub fn exact(width: u32, height: u32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
            scale: None,
            background: None,
        }
    }

    /// Set the background fill.
    pub fn with_background(mut self, bg: Option<Rgba<u8>>) -> Self {
        self.background = bg;
        self
    }
}

/// The intrinsic (author-declared) size of an SVG document, in pixels.
#[derive(Debug, Clone, Copy)]
pub struct IntrinsicSize {
    pub width: f32,
    pub height: f32,
}

/// Parse SVG bytes into a render tree, loading system fonts for text support.
fn parse_tree(svg: &[u8]) -> Result<usvg::Tree> {
    let mut opt = usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();
    usvg::Tree::from_data(svg, &opt).context("failed to parse SVG")
}

/// Return the intrinsic size of an SVG without rasterizing it.
pub fn intrinsic_size(svg: &[u8]) -> Result<IntrinsicSize> {
    let tree = parse_tree(svg)?;
    let size = tree.size();
    Ok(IntrinsicSize {
        width: size.width(),
        height: size.height(),
    })
}

/// Resolve the final target pixel dimensions from options and intrinsic size.
fn target_dimensions(intrinsic: IntrinsicSize, opt: &RenderOptions) -> Result<(u32, u32)> {
    let iw = intrinsic.width.max(1.0);
    let ih = intrinsic.height.max(1.0);
    let (w, h) = match (opt.width, opt.height) {
        (Some(w), Some(h)) => (w as f32, h as f32),
        (Some(w), None) => (w as f32, w as f32 * ih / iw),
        (None, Some(h)) => (h as f32 * iw / ih, h as f32),
        (None, None) => {
            let s = opt.scale.unwrap_or(1.0);
            (iw * s, ih * s)
        }
    };
    let w = w.round().max(1.0) as u32;
    let h = h.round().max(1.0) as u32;
    anyhow::ensure!(
        w <= 1 << 16 && h <= 1 << 16,
        "target dimensions {w}x{h} exceed the 65536px limit"
    );
    Ok((w, h))
}

/// Rasterize `svg` to an RGBA image according to `opt`.
pub fn rasterize(svg: &[u8], opt: &RenderOptions) -> Result<RgbaImage> {
    let tree = parse_tree(svg)?;
    let size = tree.size();
    let intrinsic = IntrinsicSize {
        width: size.width(),
        height: size.height(),
    };
    let (tw, th) = target_dimensions(intrinsic, opt)?;

    let mut pixmap = tiny_skia::Pixmap::new(tw, th)
        .ok_or_else(|| anyhow::anyhow!("failed to allocate {tw}x{th} pixmap"))?;

    if let Some(bg) = opt.background {
        pixmap.fill(tiny_skia::Color::from_rgba8(bg[0], bg[1], bg[2], bg[3]));
    }

    let sx = tw as f32 / intrinsic.width.max(1.0);
    let sy = th as f32 / intrinsic.height.max(1.0);
    let transform = tiny_skia::Transform::from_scale(sx, sy);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    Ok(pixmap_to_rgba(&pixmap))
}

/// Convert a premultiplied tiny-skia pixmap into a straight-alpha RGBA image.
fn pixmap_to_rgba(pixmap: &tiny_skia::Pixmap) -> RgbaImage {
    let (w, h) = (pixmap.width(), pixmap.height());
    let mut out = RgbaImage::new(w, h);
    for (src, dst) in pixmap.pixels().iter().zip(out.pixels_mut()) {
        let demul = src.demultiply();
        *dst = Rgba([demul.red(), demul.green(), demul.blue(), demul.alpha()]);
    }
    out
}
