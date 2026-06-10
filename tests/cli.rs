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
        .stdout(predicate::str::contains("# Skill: agent-trace"))
        .stdout(predicate::str::contains("judge"));
}
