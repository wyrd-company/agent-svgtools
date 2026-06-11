//! Shared helpers for the integration tests.

use assert_cmd::Command;
use std::path::PathBuf;

/// A fresh `agent-svgtools` command bound to the built binary.
pub fn bin() -> Command {
    Command::cargo_bin("agent-svgtools").expect("binary builds")
}

/// Absolute path to a file under `tests/fixtures`.
pub fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Render the `logo.svg` fixture to a PNG inside `dir` and return its path.
/// Used as the reference image by several tests.
pub fn render_reference(dir: &std::path::Path) -> PathBuf {
    let png = dir.join("reference.png");
    bin()
        .arg("render")
        .arg(fixture("logo.svg"))
        .arg(&png)
        .arg("--width")
        .arg("128")
        .assert()
        .success();
    png
}
