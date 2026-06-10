//! SVG source loading.

use anyhow::{Context, Result};
use std::path::Path;

/// Read raw SVG bytes from disk. Parsing is the renderer's responsibility.
pub fn read_svg_bytes(path: &Path) -> Result<Vec<u8>> {
    std::fs::read(path).with_context(|| format!("failed to read SVG: {}", path.display()))
}
