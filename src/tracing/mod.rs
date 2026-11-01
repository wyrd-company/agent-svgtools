//! Raster→vector tracing via the embedded `vtracer`/`visioncortex` engine.
//!
//! Exposes the tuning knobs an agent actually reaches for and returns both the
//! SVG text and a machine-readable summary (path/color counts) so the result
//! can be scored or inspected without re-parsing the SVG.

use anyhow::Result;
use image::RgbaImage;
use serde::Serialize;
use std::collections::HashSet;
use visioncortex::PathSimplifyMode;
use vtracer::{ColorImage, ColorMode, Config, Hierarchical};

/// Curve-fitting mode for traced paths.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum CurveMode {
    /// Keep pixel-accurate, axis-aligned edges.
    Pixel,
    /// Fit straight polygon segments.
    Polygon,
    /// Fit smooth splines (vtracer's default).
    Spline,
}

/// How colors are layered in the output.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum LayerMode {
    /// Opaque shapes stacked back-to-front.
    Stacked,
    /// Non-overlapping cutout shapes.
    Cutout,
}

/// Color vs. black-and-white tracing.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum Coloring {
    Color,
    Binary,
}

/// All tunable tracing parameters, with vtracer's defaults.
#[derive(Debug, Clone)]
pub struct TraceOptions {
    pub coloring: Coloring,
    pub layer: LayerMode,
    pub curve: CurveMode,
    pub filter_speckle: usize,
    pub color_precision: i32,
    pub layer_difference: i32,
    pub corner_threshold: i32,
    pub length_threshold: f64,
    pub splice_threshold: i32,
    pub max_iterations: usize,
    pub path_precision: Option<u32>,
}

impl Default for TraceOptions {
    fn default() -> Self {
        Self {
            coloring: Coloring::Color,
            layer: LayerMode::Stacked,
            curve: CurveMode::Spline,
            filter_speckle: 4,
            color_precision: 6,
            layer_difference: 16,
            corner_threshold: 60,
            length_threshold: 4.0,
            splice_threshold: 45,
            max_iterations: 10,
            path_precision: Some(2),
        }
    }
}

impl TraceOptions {
    fn to_config(&self) -> Config {
        Config {
            color_mode: match self.coloring {
                Coloring::Color => ColorMode::Color,
                Coloring::Binary => ColorMode::Binary,
            },
            hierarchical: match self.layer {
                LayerMode::Stacked => Hierarchical::Stacked,
                LayerMode::Cutout => Hierarchical::Cutout,
            },
            mode: match self.curve {
                CurveMode::Pixel => PathSimplifyMode::None,
                CurveMode::Polygon => PathSimplifyMode::Polygon,
                CurveMode::Spline => PathSimplifyMode::Spline,
            },
            filter_speckle: self.filter_speckle,
            color_precision: self.color_precision,
            layer_difference: self.layer_difference,
            corner_threshold: self.corner_threshold,
            length_threshold: self.length_threshold,
            splice_threshold: self.splice_threshold,
            max_iterations: self.max_iterations,
            path_precision: self.path_precision,
        }
    }
}

/// The product of a trace: the SVG document plus a structural summary.
#[derive(Debug, Clone)]
pub struct TraceResult {
    pub svg: String,
    pub summary: TraceSummary,
}

/// Machine-readable trace statistics.
#[derive(Debug, Clone, Serialize)]
pub struct TraceSummary {
    pub width: u32,
    pub height: u32,
    /// Number of `<path>` elements emitted.
    pub path_count: usize,
    /// Number of distinct fill colors used.
    pub color_count: usize,
}

/// Trace `img` into an SVG using `opts`.
pub fn trace(img: &RgbaImage, opts: &TraceOptions) -> Result<TraceResult> {
    let (w, h) = img.dimensions();
    let color_image = ColorImage {
        pixels: img.as_raw().clone(),
        width: w as usize,
        height: h as usize,
    };

    let svg_file = vtracer::convert(color_image, opts.to_config())
        .map_err(|e| anyhow::anyhow!("vtracer failed: {e}"))?;

    let path_count = svg_file.paths.len();
    let color_count = svg_file
        .paths
        .iter()
        .map(|p| (p.color.r, p.color.g, p.color.b, p.color.a))
        .collect::<HashSet<_>>()
        .len();

    Ok(TraceResult {
        svg: svg_file.to_string(),
        summary: TraceSummary {
            width: w,
            height: h,
            path_count,
            color_count,
        },
    })
}
