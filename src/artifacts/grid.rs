//! Coordinate gridlines and axis labels.
//!
//! Rather than ship a bitmap font, the grid is authored as a small SVG overlay
//! and rasterized with the project's own renderer. This keeps a single text/
//! vector path, gives crisp labels wherever system fonts are present, and
//! degrades gracefully (lines still render) where they are not.

use crate::artifacts::compose;
use crate::render::{rasterize, RenderOptions};
use anyhow::Result;
use image::RgbaImage;

/// Choose a "nice" gridline spacing (1/2/5 × 10ⁿ) targeting ~10 divisions.
pub fn nice_spacing(width: u32, height: u32) -> u32 {
    let longest = width.max(height).max(1) as f64;
    let raw = longest / 10.0;
    let magnitude = 10f64.powf(raw.log10().floor());
    let normalized = raw / magnitude;
    let step = if normalized < 1.5 {
        1.0
    } else if normalized < 3.5 {
        2.0
    } else if normalized < 7.5 {
        5.0
    } else {
        10.0
    };
    (step * magnitude).max(1.0) as u32
}

/// Build an SVG `<g>` fragment of gridlines + labels for a panel of size
/// `w`×`h` whose top-left sits at `(off_x, off_y)` on the canvas. Labels show
/// panel-local pixel coordinates so they map to the SVG the agent is authoring.
pub fn grid_group(off_x: u32, off_y: u32, w: u32, h: u32, spacing: u32) -> String {
    let mut s = format!(r#"<g transform="translate({off_x},{off_y})">"#);
    let stroke = r##"stroke="#ff00ff" stroke-opacity="0.5" stroke-width="1""##;
    let font = r##"font-family="monospace" font-size="10" fill="#ff00ff" fill-opacity="0.9""##;

    let mut x = 0;
    while x <= w {
        s.push_str(&format!(
            r#"<line x1="{x}" y1="0" x2="{x}" y2="{h}" {stroke}/>"#
        ));
        if x > 0 {
            s.push_str(&format!(r#"<text x="{}" y="11" {font}>{x}</text>"#, x + 2));
        }
        x += spacing;
    }

    let mut y = 0;
    while y <= h {
        s.push_str(&format!(
            r#"<line x1="0" y1="{y}" x2="{w}" y2="{y}" {stroke}/>"#
        ));
        if y > 0 {
            s.push_str(&format!(r#"<text x="2" y="{}" {font}>{y}</text>"#, y + 11));
        }
        y += spacing;
    }
    s.push_str("</g>");
    s
}

/// Wrap SVG `body` into a complete transparent document of the given size.
pub fn wrap_svg(width: u32, height: u32, body: &str) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">{body}</svg>"#
    )
}

/// Rasterize an SVG `body` to a transparent RGBA overlay of the given size.
pub fn render_overlay(width: u32, height: u32, body: &str) -> Result<RgbaImage> {
    let svg = wrap_svg(width, height, body);
    rasterize(svg.as_bytes(), &RenderOptions::exact(width, height))
}

/// Draw a coordinate grid over a single image, in place.
pub fn apply_grid(img: &mut RgbaImage) -> Result<()> {
    let (w, h) = img.dimensions();
    let spacing = nice_spacing(w, h);
    let body = grid_group(0, 0, w, h, spacing);
    let overlay = render_overlay(w, h, &body)?;
    compose::over(img, &overlay, 0, 0);
    Ok(())
}
