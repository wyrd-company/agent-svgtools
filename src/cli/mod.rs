//! Command-line surface: the top-level parser and subcommand dispatch.

use crate::commands;
use crate::commands::shared::OutputCtx;
use crate::output::OutputFormat;
use anyhow::Result;
use clap::{Parser, Subcommand};

/// agent-trace: close the feedback loop for AI agents authoring SVGs from
/// raster references — render, score, localize, and visualize.
#[derive(Debug, Parser)]
#[command(
    name = "agent-trace",
    version,
    about = "Machine-first SVG-vs-raster scoring for AI coding agents.",
    long_about = "Render SVGs, score them against raster references (pixel match + SSIM), \
report ranked error regions, and emit vision-model-friendly comparison images. \
Default output is TOON; use --format json|jsonl or --pretty as needed. \
Run `agent-trace skill` for the full agent workflow guide."
)]
pub struct Cli {
    /// Output format for machine-readable results.
    #[arg(long, value_enum, default_value_t = OutputFormat::default(), global = true)]
    pub format: OutputFormat,

    /// Pretty-print JSON output (only affects --format json).
    #[arg(long, global = true)]
    pub pretty: bool,

    #[command(subcommand)]
    pub command: Command,
}

/// The subcommands. Each variant carries that command's own argument struct.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Render an SVG candidate and score it against a raster reference.
    Judge(commands::judge::Args),
    /// Rasterize an SVG to a PNG.
    Render(commands::render::Args),
    /// Trace a raster image into a starting-point SVG.
    Trace(commands::trace::Args),
    /// Extract the dominant color palette of an image.
    Colors(commands::colors::Args),
    /// Compare two raster images.
    Compare(commands::compare::Args),
    /// Print the agent-facing skill document.
    Skill(commands::skill::Args),
}

impl Cli {
    /// Dispatch to the selected command, returning the process exit code.
    pub fn run(&self) -> Result<i32> {
        let ctx = OutputCtx {
            format: self.format,
            pretty: self.pretty,
        };
        match &self.command {
            Command::Judge(a) => commands::judge::run(a, &ctx),
            Command::Render(a) => commands::render::run(a, &ctx),
            Command::Trace(a) => commands::trace::run(a, &ctx),
            Command::Colors(a) => commands::colors::run(a, &ctx),
            Command::Compare(a) => commands::compare::run(a, &ctx),
            Command::Skill(a) => commands::skill::run(a, &ctx),
        }
    }
}
