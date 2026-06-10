//! Output serialization for machine-first consumption.
//!
//! Every command produces a value that implements [`Report`]. The [`emit`]
//! function renders it in the format the caller selected on the command line.
//!
//! Format philosophy (see `docs/decisions` and the README): the default is
//! TOON because the primary reader is an LLM agent in a tight iterate-and-score
//! loop, and TOON encodes the same structure in materially fewer tokens than
//! JSON. JSON and JSONL remain first-class for callers that need a universal
//! parser or a streaming, line-delimited record format.

mod format;

pub use format::OutputFormat;

use anyhow::Result;
use serde::Serialize;
use std::io::Write;

/// A command result that can be rendered in any [`OutputFormat`].
///
/// The blanket structure is: a report serializes as a single object for TOON
/// and JSON, and as one-or-more line-delimited records for JSONL. Most reports
/// are a single record; commands with a natural list dimension (e.g. `judge`'s
/// diff regions) override [`Report::jsonl_records`] to stream a header record
/// followed by one record per element.
pub trait Report: Serialize {
    /// Records to emit, one per line, when the format is JSONL.
    ///
    /// Defaults to a single record wrapping the whole report. Each returned
    /// value MUST be a JSON object so the stream is self-describing.
    fn jsonl_records(&self) -> Vec<serde_json::Value> {
        vec![serde_json::to_value(self).unwrap_or(serde_json::Value::Null)]
    }
}

/// Render `report` to `writer` in the requested format.
pub fn emit<W: Write, R: Report>(
    writer: &mut W,
    report: &R,
    format: OutputFormat,
    pretty: bool,
) -> Result<()> {
    match format {
        OutputFormat::Toon => {
            let encoded = toon_format::encode_default(report)
                .map_err(|e| anyhow::anyhow!("TOON encoding failed: {e}"))?;
            writeln!(writer, "{}", encoded.trim_end())?;
        }
        OutputFormat::Json => {
            if pretty {
                writeln!(writer, "{}", serde_json::to_string_pretty(report)?)?;
            } else {
                writeln!(writer, "{}", serde_json::to_string(report)?)?;
            }
        }
        OutputFormat::Jsonl => {
            for record in report.jsonl_records() {
                writeln!(writer, "{}", serde_json::to_string(&record)?)?;
            }
        }
    }
    Ok(())
}

/// Convenience: emit a report to stdout.
pub fn emit_stdout<R: Report>(report: &R, format: OutputFormat, pretty: bool) -> Result<()> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    emit(&mut lock, report, format, pretty)
}
