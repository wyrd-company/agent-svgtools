//! Plumbing shared by the comparison commands (`judge`, `compare`).

mod artifacts;

pub use artifacts::{write_artifacts, ArtifactPaths, ArtifactRequest};

use crate::compare::RegionConfig;

/// Output context threaded through every command's `run`.
#[derive(Debug, Clone, Copy)]
pub struct OutputCtx {
    pub format: crate::output::OutputFormat,
    pub pretty: bool,
}

/// Build a [`RegionConfig`] from auto-defaults plus optional CLI overrides.
pub fn region_cfg(width: u32, height: u32, block: Option<u32>, max: Option<usize>) -> RegionConfig {
    let mut cfg = RegionConfig::auto(width, height);
    if let Some(b) = block {
        cfg.block = b.max(1);
    }
    if let Some(m) = max {
        cfg.max_regions = m;
    }
    cfg
}
