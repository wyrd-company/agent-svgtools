//! `agent-svgtools` binary entry point.
//!
//! Thin by design: parse arguments, dispatch into the library, and translate
//! the result into a process exit code. All real work lives in the crate's
//! modules (see `src/lib.rs`).
//!
//! Exit codes: 0 success, 1 usage/IO error, 2 threshold not met (judge/compare).

use agent_svgtools::cli::Cli;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.run() {
        Ok(code) => ExitCode::from(code as u8),
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(1)
        }
    }
}
