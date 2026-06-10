//! `agent-trace render` — plain SVG rasterization to PNG.

use crate::commands::shared::OutputCtx;
use crate::io::{read_svg_bytes, save_png};
use crate::output::{emit_stdout, Report};
use crate::render::{rasterize, RenderOptions};
use crate::util::color::parse_rgba;
use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;

/// Rasterize an SVG to a PNG. With both --width and --height the SVG is
/// stretched to fill; with one, aspect ratio is preserved; with neither, the
/// SVG's intrinsic size (times --scale) is used.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Input SVG file.
    pub input: PathBuf,
    /// Output PNG path (parent directories are created).
    pub output: PathBuf,
    /// Target width in pixels.
    #[arg(long)]
    pub width: Option<u32>,
    /// Target height in pixels.
    #[arg(long)]
    pub height: Option<u32>,
    /// Uniform scale of the intrinsic size (ignored if width/height given).
    #[arg(long)]
    pub scale: Option<f32>,
    /// Background fill: #rgb, #rrggbb, #rrggbbaa, a name (white/black), or
    /// transparent (the default).
    #[arg(long)]
    pub background: Option<String>,
}

#[derive(Serialize)]
struct RenderReport {
    input: String,
    output: String,
    width: u32,
    height: u32,
}

impl Report for RenderReport {}

pub fn run(args: &Args, ctx: &OutputCtx) -> Result<i32> {
    let svg = read_svg_bytes(&args.input)?;
    let background = match &args.background {
        Some(s) => Some(parse_rgba(s)?),
        None => None,
    };
    let opt = RenderOptions {
        width: args.width,
        height: args.height,
        scale: args.scale,
        background,
    };
    let img = rasterize(&svg, &opt)?;
    let (width, height) = img.dimensions();
    save_png(&img, &args.output)?;

    let report = RenderReport {
        input: args.input.display().to_string(),
        output: args.output.display().to_string(),
        width,
        height,
    };
    emit_stdout(&report, ctx.format, ctx.pretty)?;
    Ok(0)
}
