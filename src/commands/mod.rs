//! One module per CLI subcommand.
//!
//! Each command module owns its clap argument struct and a `run` entry point,
//! keeping argument definitions next to the logic they drive. Cross-command
//! plumbing (region config, artifact writing) lives in [`shared`].

pub mod colors;
pub mod compare;
pub mod judge;
pub mod render;
pub mod shared;
pub mod skill;
pub mod trace;
