//! Generate and persist comparison artifact images on request.

use crate::artifacts;
use crate::io::save_png;
use anyhow::Result;
use image::RgbaImage;
use serde::Serialize;
use std::path::Path;

/// Which artifacts to produce, and where to write them.
#[derive(Debug, Default, Clone)]
pub struct ArtifactRequest<'a> {
    pub diff_image: Option<&'a Path>,
    pub side_by_side: Option<&'a Path>,
    pub blend: Option<&'a Path>,
    pub grid: bool,
}

impl ArtifactRequest<'_> {
    fn any(&self) -> bool {
        self.diff_image.is_some() || self.side_by_side.is_some() || self.blend.is_some()
    }
}

/// Paths of the artifacts that were written, for inclusion in the report.
#[derive(Debug, Default, Clone, Serialize)]
pub struct ArtifactPaths {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_by_side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend: Option<String>,
}

/// Generate every requested artifact comparing `reference` to `rendered`.
pub fn write_artifacts(
    reference: &RgbaImage,
    rendered: &RgbaImage,
    req: &ArtifactRequest,
) -> Result<ArtifactPaths> {
    let mut paths = ArtifactPaths::default();
    if !req.any() {
        return Ok(paths);
    }

    if let Some(path) = req.diff_image {
        let mut img = artifacts::diff_image(reference, rendered)?;
        if req.grid {
            artifacts::apply_grid(&mut img)?;
        }
        save_png(&img, path)?;
        paths.diff_image = Some(path.display().to_string());
    }

    if let Some(path) = req.side_by_side {
        // side_by_side draws its own per-panel grid when requested.
        let img = artifacts::side_by_side(reference, rendered, req.grid)?;
        save_png(&img, path)?;
        paths.side_by_side = Some(path.display().to_string());
    }

    if let Some(path) = req.blend {
        let mut img = artifacts::blend(reference, rendered)?;
        if req.grid {
            artifacts::apply_grid(&mut img)?;
        }
        save_png(&img, path)?;
        paths.blend = Some(path.display().to_string());
    }

    Ok(paths)
}
