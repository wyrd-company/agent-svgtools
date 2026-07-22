//! `agent-svgtools compare` — score two raster images with the same engine as
//! `judge` (useful when the agent has already rendered something itself).

mod report;

use crate::commands::shared::{region_cfg, write_artifacts, ArtifactRequest, OutputCtx};
use crate::compare::{compare, compare_foreground};
use crate::io::load_rgba;
use crate::output::emit_stdout;
use anyhow::Result;
use image::imageops::FilterType;
use report::{CompareReport, ThresholdResult};
use std::path::PathBuf;

/// Compare two raster images. IMAGE_B is resized to IMAGE_A's dimensions if they
/// differ (a warning is emitted).
#[derive(Debug, clap::Args)]
pub struct Args {
    /// First (reference) image.
    pub image_a: PathBuf,
    /// Second (candidate) image.
    pub image_b: PathBuf,

    /// Write a diff heatmap PNG to this path.
    #[arg(long)]
    pub diff_image: Option<PathBuf>,
    /// Write a labeled side-by-side PNG to this path.
    #[arg(long)]
    pub side_by_side: Option<PathBuf>,
    /// Write a 50% onion-skin blend PNG to this path.
    #[arg(long)]
    pub blend: Option<PathBuf>,
    /// Overlay coordinate gridlines + axis labels on generated images.
    #[arg(long)]
    pub grid: bool,
    /// Exit with code 2 if the combined score is below this value (0–100).
    #[arg(long)]
    pub threshold: Option<f64>,
    /// Score the union of non-transparent pixels with alpha agreement,
    /// preventing empty canvas or an opaque replacement from inflating similarity.
    #[arg(long)]
    pub foreground: bool,
    /// Region detection block size in pixels (default: auto).
    #[arg(long)]
    pub block: Option<u32>,
    /// Maximum number of error regions to report.
    #[arg(long)]
    pub max_regions: Option<usize>,
}

pub fn run(args: &Args, ctx: &OutputCtx) -> Result<i32> {
    let image_a = load_rgba(&args.image_a)?;
    let mut image_b = load_rgba(&args.image_b)?;
    let (aw, ah) = image_a.dimensions();

    let mut warnings = Vec::new();
    if image_b.dimensions() != (aw, ah) {
        let (bw, bh) = image_b.dimensions();
        warnings.push(format!(
            "dimension mismatch: image-a {aw}x{ah} vs image-b {bw}x{bh}; image-b was resized to {aw}x{ah}"
        ));
        image_b = image::imageops::resize(&image_b, aw, ah, FilterType::Triangle);
    }

    let cfg = region_cfg(aw, ah, args.block, args.max_regions);
    let comparison = if args.foreground {
        compare_foreground(&image_a, &image_b, &cfg)?
    } else {
        compare(&image_a, &image_b, &cfg)?
    };

    let artifacts = write_artifacts(
        &image_a,
        &image_b,
        &ArtifactRequest {
            diff_image: args.diff_image.as_deref(),
            side_by_side: args.side_by_side.as_deref(),
            blend: args.blend.as_deref(),
            grid: args.grid,
        },
    )?;
    let artifacts =
        (args.diff_image.is_some() || args.side_by_side.is_some() || args.blend.is_some())
            .then_some(artifacts);

    let threshold = args.threshold.map(|value| ThresholdResult {
        value,
        passed: comparison.combined >= value,
    });
    let exit = match &threshold {
        Some(t) if !t.passed => 2,
        _ => 0,
    };

    let report = CompareReport::new(
        args.image_a.display().to_string(),
        args.image_b.display().to_string(),
        comparison,
        warnings,
        artifacts,
        threshold,
    );
    emit_stdout(&report, ctx.format, ctx.pretty)?;
    Ok(exit)
}
