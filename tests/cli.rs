//! End-to-end CLI behavior: subcommands, flags, exit codes, artifacts.

mod common;

use common::{bin, fixture, render_reference};
use predicates::prelude::*;

#[test]
fn render_writes_png_and_reports_dimensions() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("out.png");
    bin()
        .args(["render"])
        .arg(fixture("logo.svg"))
        .arg(&out)
        .args(["--width", "256"])
        .assert()
        .success()
        .stdout(predicate::str::contains("width: 256"));
    assert!(out.exists());
}

#[test]
fn judge_emits_scores_and_writes_artifacts() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());
    let diff = dir.path().join("diff.png");
    let sbs = dir.path().join("sbs.png");
    let blend = dir.path().join("blend.png");

    bin()
        .arg("judge")
        .arg(&reference)
        .arg(fixture("logo.svg"))
        .arg("--diff-image")
        .arg(&diff)
        .arg("--side-by-side")
        .arg(&sbs)
        .arg("--blend")
        .arg(&blend)
        .arg("--grid")
        .assert()
        .success()
        .stdout(predicate::str::contains("combined:"));

    assert!(diff.exists() && sbs.exists() && blend.exists());
}

#[test]
fn judge_threshold_controls_exit_code() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());

    // Identical content clears any reasonable threshold.
    bin()
        .arg("judge")
        .arg(&reference)
        .arg(fixture("logo.svg"))
        .args(["--threshold", "95"])
        .assert()
        .success();

    // A candidate that genuinely differs cannot clear a perfect score, so the
    // threshold check fails with exit code 2.
    bin()
        .arg("judge")
        .arg(&reference)
        .arg(fixture("candidate.svg"))
        .args(["--threshold", "100.0"])
        .assert()
        .code(2);
}

#[test]
fn judge_foreground_reports_selected_score_scope() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());

    bin()
        .arg("judge")
        .arg(&reference)
        .arg(fixture("candidate.svg"))
        .args(["--foreground", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"scope\":\"foreground\""));
}

#[test]
fn compare_foreground_reports_selected_score_scope() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());

    bin()
        .arg("compare")
        .arg(&reference)
        .arg(&reference)
        .args(["--foreground", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"scope\":\"foreground\""));
}

#[test]
fn missing_input_is_usage_error_exit_1() {
    bin()
        .arg("judge")
        .arg("/nonexistent/reference.png")
        .arg(fixture("logo.svg"))
        .assert()
        .code(1)
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn colors_reports_palette() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());
    bin()
        .arg("colors")
        .arg(&reference)
        .args(["--max-colors", "4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#1d4ed8").or(predicate::str::contains("1d4ed8")));
}

#[test]
fn inspect_reports_structure_palette_and_safety() {
    bin()
        .arg("inspect")
        .arg(fixture("candidate.svg"))
        .args(["--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"authored_graphics\""))
        .stdout(predicate::str::contains("\"palette\""))
        .stdout(predicate::str::contains("\"vector_only\":true"));
}

#[test]
fn inspect_reports_embedded_raster_and_external_resources() {
    let dir = tempfile::tempdir().unwrap();
    let svg = dir.path().join("unsafe.svg");
    std::fs::write(
        &svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" onload="alert(1)">
             <image href="data:image/png;base64,AA=="/>
             <use href="https://example.invalid/icon.svg#shape"/>
             <use href="sprite.svg#shape"/>
           </svg>"#,
    )
    .unwrap();

    bin()
        .arg("inspect")
        .arg(svg)
        .args(["--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"vector_only\":false"))
        .stdout(predicate::str::contains("\"prohibited_tags\":[\"image\"]"))
        .stdout(predicate::str::contains(
            "\"scripted_attributes\":[\"onload\"]",
        ))
        .stdout(predicate::str::contains("\"embedded_data\":true"))
        .stdout(predicate::str::contains("https://example.invalid"))
        .stdout(predicate::str::contains("sprite.svg#shape"));
}

#[test]
fn trace_to_stdout_emits_svg() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());
    bin()
        .arg("trace")
        .arg(&reference)
        .assert()
        .success()
        .stdout(predicate::str::contains("<svg"));
}

#[test]
fn skill_prints_document() {
    bin()
        .arg("skill")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("---\nname: agent-svgtools\n"))
        .stdout(predicate::str::contains(
            "# Clean Raster-to-SVG Reconstruction",
        ))
        .stdout(predicate::str::contains("judge"));
}
