//! agent-trace: a machine-first toolkit for the SVG-from-raster feedback loop.
//!
//! The crate is split into focused subsystems, each in its own module:
//!
//! - [`io`]      — load/save raster images and read SVG sources.
//! - [`render`]  — rasterize SVG documents to RGBA buffers.
//! - [`compare`] — the scoring engine (pixel match, SSIM, localized regions).
//! - [`palette`] — dominant-color extraction.
//! - [`tracing`] — raster→vector tracing via the embedded vtracer/visioncortex.
//! - [`artifacts`] — vision-model-friendly comparison images.
//! - [`output`]  — render any command result as TOON/JSON/JSONL.
//! - [`skill`]   — the agent-facing skill document.
//! - [`commands`] — one module per CLI subcommand, orchestrating the above.
//! - [`cli`]     — the top-level parser and subcommand dispatch.
//!
//! The library is consumed by the thin `agent-trace` binary (`src/main.rs`),
//! and exercised directly by the integration tests.

pub mod artifacts;
pub mod cli;
pub mod commands;
pub mod compare;
pub mod io;
pub mod output;
pub mod palette;
pub mod render;
pub mod skill;
pub mod tracing;
pub mod util;
