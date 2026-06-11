//! Labeled side-by-side comparison: `reference | rendered`.

use crate::artifacts::compose::{flatten_over_white, over};
use crate::artifacts::grid;
use anyhow::Result;
use image::{Rgba, RgbaImage};

const LABEL_BAND: u32 = 18;
const GAP: u32 = 10;

/// Compose the reference and rendered candidate side by side on a white canvas,
/// each captioned, with an optional coordinate grid drawn over each panel.
pub fn side_by_side(
    reference: &RgbaImage,
    rendered: &RgbaImage,
    with_grid: bool,
) -> Result<RgbaImage> {
    let r = flatten_over_white(reference);
    let c = flatten_over_white(rendered);
    let (w1, h1) = r.dimensions();
    let (w2, h2) = c.dimensions();

    let canvas_w = w1 + GAP + w2;
    let canvas_h = LABEL_BAND + h1.max(h2);
    let mut canvas = RgbaImage::from_pixel(canvas_w, canvas_h, Rgba([255, 255, 255, 255]));

    over(&mut canvas, &r, 0, LABEL_BAND as i64);
    over(&mut canvas, &c, (w1 + GAP) as i64, LABEL_BAND as i64);

    let mut body = String::new();
    // Panel captions.
    let label = r##"font-family="monospace" font-size="12" fill="#222222""##;
    body.push_str(&format!(r#"<text x="4" y="13" {label}>reference</text>"#));
    body.push_str(&format!(
        r#"<text x="{}" y="13" {label}>rendered</text>"#,
        w1 + GAP + 4
    ));
    // Divider line in the gap.
    let divx = w1 + GAP / 2;
    body.push_str(&format!(
        r##"<line x1="{divx}" y1="{LABEL_BAND}" x2="{divx}" y2="{canvas_h}" stroke="#cccccc" stroke-width="1"/>"##
    ));

    if with_grid {
        let s1 = grid::nice_spacing(w1, h1);
        let s2 = grid::nice_spacing(w2, h2);
        body.push_str(&grid::grid_group(0, LABEL_BAND, w1, h1, s1));
        body.push_str(&grid::grid_group(w1 + GAP, LABEL_BAND, w2, h2, s2));
    }

    let overlay = grid::render_overlay(canvas_w, canvas_h, &body)?;
    over(&mut canvas, &overlay, 0, 0);
    Ok(canvas)
}
