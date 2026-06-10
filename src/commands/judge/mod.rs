//! `agent-trace judge` — render an SVG candidate and score it against a
//! raster reference.

mod report;

use crate::commands::shared::{region_cfg, write_artifacts, ArtifactRequest, OutputCtx};
use crate::compare::compare;
use crate::io::{load_rgba, read_svg_bytes};
use crate::output::emit_stdout;
use crate::render::{intrinsic_size, rasterize, RenderOptions};
use anyhow::Result;
use report::{JudgeReport, ThresholdResult};
use std::path::PathBuf;

/// Render CANDIDATE at the reference's resolution and score the match: pixel
/// match %, SSIM, a combined score, and ranked error regions.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Reference raster image.
    pub reference: PathBuf,
    /// Candidate SVG file.
    pub candidate: PathBuf,

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
    /// Region detection block size in pixels (default: auto).
    #[arg(long)]
    pub block: Option<u32>,
    /// Maximum number of error regions to report.
    #[arg(long)]
    pub max_regions: Option<usize>,
}

pub fn run(args: &Args, ctx: &OutputCtx) -> Result<i32> {
    let reference = load_rgba(&args.reference)?;
    let (rw, rh) = reference.dimensions();

    let svg = read_svg_bytes(&args.candidate)?;
    let mut warnings = Vec::new();
    if let Ok(intrinsic) = intrinsic_size(&svg) {
        if let Some(w) = aspect_warning((rw, rh), (intrinsic.width, intrinsic.height)) {
            warnings.push(w);
        }
    }

    // Render the candidate at the reference resolution so they align.
    let rendered = rasterize(&svg, &RenderOptions::exact(rw, rh))?;

    let cfg = region_cfg(rw, rh, args.block, args.max_regions);
    let comparison = compare(&reference, &rendered, &cfg)?;

    let artifacts = write_artifacts(
        &reference,
        &rendered,
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

    let report = JudgeReport::new(
        args.reference.display().to_string(),
        (rw, rh),
        args.candidate.display().to_string(),
        comparison,
        warnings,
        artifacts,
        threshold,
    );
    emit_stdout(&report, ctx.format, ctx.pretty)?;
    Ok(exit)
}

/// Warn when the reference and SVG aspect ratios diverge by more than 1%, since
/// the candidate is stretched to the reference canvas before scoring.
fn aspect_warning(reference: (u32, u32), svg: (f32, f32)) -> Option<String> {
    let ref_aspect = reference.0 as f32 / reference.1.max(1) as f32;
    let svg_aspect = svg.0 / svg.1.max(1.0);
    let rel = (ref_aspect - svg_aspect).abs() / ref_aspect.max(f32::EPSILON);
    if rel > 0.01 {
        Some(format!(
            "aspect-ratio mismatch: reference {}x{} ({:.3}) vs svg {:.0}x{:.0} ({:.3}); candidate was stretched to fit",
            reference.0, reference.1, ref_aspect, svg.0, svg.1, svg_aspect
        ))
    } else {
        None
    }
}
