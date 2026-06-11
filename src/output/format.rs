//! Output format selection.

use clap::ValueEnum;
use serde::Serialize;

/// Machine-readable output format for command results.
///
/// The default is [`OutputFormat::Toon`]. See the module docs for the rationale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Token-Oriented Object Notation: compact, LLM-friendly. The default.
    #[default]
    Toon,
    /// Standard JSON. Compact by default; pass `--pretty` for indentation.
    Json,
    /// Newline-delimited JSON records. Streams a header record followed by
    /// one record per list element (e.g. one per diff region in `judge`).
    Jsonl,
}

impl OutputFormat {
    /// Human label used in help text and diagnostics.
    pub fn as_str(self) -> &'static str {
        match self {
            OutputFormat::Toon => "toon",
            OutputFormat::Json => "json",
            OutputFormat::Jsonl => "jsonl",
        }
    }
}
