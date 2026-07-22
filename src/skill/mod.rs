//! The agent-facing skill document.
//!
//! `agent-svgtools skill` prints the repository's canonical project skill
//! verbatim so it can be installed into another project. Keeping the embedded
//! document and the dogfooded project skill as one file prevents drift.

/// The full skill document (Markdown).
pub fn skill_document() -> &'static str {
    include_str!("../../.agents/skills/agent-svgtools/SKILL.md")
}
