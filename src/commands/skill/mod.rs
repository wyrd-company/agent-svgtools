//! `agent-trace skill` — print the agent-facing skill document.

use crate::commands::shared::OutputCtx;
use crate::skill::skill_document;
use anyhow::Result;

/// Print a complete, agent-facing skill document (Markdown) to stdout.
/// Install it into a project with `agent-trace skill > SKILL.md`.
#[derive(Debug, clap::Args)]
pub struct Args {}

pub fn run(_args: &Args, _ctx: &OutputCtx) -> Result<i32> {
    print!("{}", skill_document());
    Ok(0)
}
