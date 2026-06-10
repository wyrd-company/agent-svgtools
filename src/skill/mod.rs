//! The agent-facing skill document.
//!
//! `agent-trace skill` prints this verbatim to stdout so it can be installed
//! into a project (`agent-trace skill > SKILL.md`). It is authored for an LLM
//! agent as the reader: imperative, example-heavy, no marketing.

/// The full skill document (Markdown).
pub fn skill_document() -> &'static str {
    include_str!("skill.md")
}
