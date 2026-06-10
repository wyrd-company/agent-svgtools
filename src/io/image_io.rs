//! Raster image loading and saving.

use anyhow::{Context, Result};
use image::RgbaImage;
use std::path::Path;

/// Load any supported raster image (PNG, JPEG, BMP, GIF, WebP, TIFF) as RGBA8.
pub fn load_rgba(path: &Path) -> Result<RgbaImage> {
    let img =
        image::open(path).with_context(|| format!("failed to read image: {}", path.display()))?;
    Ok(img.to_rgba8())
}

/// Write an RGBA image to `path` as PNG, creating parent directories as needed.
pub fn save_png(img: &RgbaImage, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory: {}", parent.display()))?;
        }
    }
    img.save_with_format(path, image::ImageFormat::Png)
        .with_context(|| format!("failed to write PNG: {}", path.display()))?;
    Ok(())
}
