//! `agent-svgtools trace` — raster→vector tracing via the embedded vtracer.

use crate::commands::shared::OutputCtx;
use crate::io::load_rgba;
use crate::output::{emit, Report};
use crate::tracing::{trace, Coloring, CurveMode, LayerMode, TraceOptions, TraceSummary};
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Trace a raster image into an SVG. The SVG is written to OUTPUT (or stdout if
/// omitted); a machine-readable summary is written to stdout (or stderr when
/// the SVG is going to stdout, so the SVG stream stays clean).
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Input raster image.
    pub image: PathBuf,
    /// Output SVG path. Omit to write the SVG to stdout.
    pub output: Option<PathBuf>,

    /// Color or black-and-white tracing.
    #[arg(long, value_enum, default_value = "color")]
    pub coloring: Coloring,
    /// Shape layering strategy.
    #[arg(long, value_enum, default_value = "stacked")]
    pub layer: LayerMode,
    /// Curve fitting mode.
    #[arg(long, value_enum, default_value = "spline")]
    pub curve: CurveMode,
    /// Discard speckles smaller than this many pixels.
    #[arg(long, default_value_t = 4)]
    pub filter_speckle: usize,
    /// Color quantization precision (higher = more colors).
    #[arg(long, default_value_t = 6)]
    pub color_precision: i32,
    /// Layer color difference threshold.
    #[arg(long, default_value_t = 16)]
    pub layer_difference: i32,
    /// Minimum corner angle to keep as a corner (degrees).
    #[arg(long, default_value_t = 60)]
    pub corner_threshold: i32,
    /// Minimum segment length before splicing.
    #[arg(long, default_value_t = 4.0)]
    pub length_threshold: f64,
    /// Splice threshold (degrees).
    #[arg(long, default_value_t = 45)]
    pub splice_threshold: i32,
    /// Maximum curve-fitting iterations.
    #[arg(long, default_value_t = 10)]
    pub max_iterations: usize,
    /// Numeric precision (decimal places) of path coordinates.
    #[arg(long, default_value_t = 2)]
    pub path_precision: u32,
}

impl Args {
    fn to_options(&self) -> TraceOptions {
        TraceOptions {
            coloring: self.coloring,
            layer: self.layer,
            curve: self.curve,
            filter_speckle: self.filter_speckle,
            color_precision: self.color_precision,
            layer_difference: self.layer_difference,
            corner_threshold: self.corner_threshold,
            length_threshold: self.length_threshold,
            splice_threshold: self.splice_threshold,
            max_iterations: self.max_iterations,
            path_precision: Some(self.path_precision),
        }
    }
}

impl Report for TraceSummary {}

pub fn run(args: &Args, ctx: &OutputCtx) -> Result<i32> {
    let img = load_rgba(&args.image)?;
    let result = trace(&img, &args.to_options())?;

    match &args.output {
        Some(path) => {
            std::fs::write(path, &result.svg)
                .with_context(|| format!("failed to write SVG: {}", path.display()))?;
            // SVG went to a file, so the summary owns stdout.
            crate::output::emit_stdout(&result.summary, ctx.format, ctx.pretty)?;
        }
        None => {
            // SVG owns stdout; the summary goes to stderr to keep stdout pure.
            print!("{}", result.svg);
            let mut stderr = std::io::stderr();
            emit(&mut stderr, &result.summary, ctx.format, ctx.pretty)?;
        }
    }
    Ok(0)
}
