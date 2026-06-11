//! End-to-end: render a logo, trace it back to SVG, and judge the trace against
//! the original raster — asserting the round trip clears a quality floor.

mod common;

use common::{bin, render_reference};
use serde_json::Value;

#[test]
fn trace_then_judge_clears_quality_floor() {
    let dir = tempfile::tempdir().unwrap();

    // 1. A reference raster (rendered from the logo fixture).
    let reference = render_reference(dir.path());

    // 2. Trace it back into a starting-point SVG.
    let traced = dir.path().join("traced.svg");
    bin()
        .arg("trace")
        .arg(&reference)
        .arg(&traced)
        .assert()
        .success();
    assert!(traced.exists());

    // 3. Judge the traced SVG against the original raster.
    let out = bin()
        .arg("judge")
        .arg(&reference)
        .arg(&traced)
        .args(["--format", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let report: Value = serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();

    // A faithful trace of a flat-color logo should reproduce it well.
    let combined = report["scores"]["combined"].as_f64().unwrap();
    assert!(
        combined >= 80.0,
        "trace->judge combined score {combined} fell below the 80.0 floor"
    );
}
