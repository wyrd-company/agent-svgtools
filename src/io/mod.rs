//! Filesystem IO for raster images and SVG sources.
//!
//! Kept deliberately thin and separate from the rendering and comparison
//! engines so those operate purely on in-memory buffers and stay testable.

mod image_io;
mod svg_io;

pub use image_io::{load_rgba, save_png};
pub use svg_io::read_svg_bytes;
