//! Vision-model-friendly comparison images.
//!
//! Every generator returns an in-memory [`image::RgbaImage`]; persisting them is
//! the caller's responsibility (artifacts are only ever written to explicit
//! paths supplied by the user, never to implicit temp files).

mod blend;
mod compose;
mod diff_image;
mod grid;
mod side_by_side;

pub use blend::blend;
pub use diff_image::diff_image;
pub use grid::apply_grid;
pub use side_by_side::side_by_side;
