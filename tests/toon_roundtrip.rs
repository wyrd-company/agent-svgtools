//! TOON round-trip tests.
//!
//! The default output is TOON, encoded by the `toon-format` crate. These tests
//! run real commands, decode the TOON output with the same crate in strict
//! mode, and compare the result with the `--format json` output of the same
//! command. A `toon-format` update that changes the wire format is accepted; an
//! update whose output no longer decodes to the same value fails here.

mod common;

use common::{bin, fixture, render_reference};
use serde_json::Value;
use std::ffi::OsString;

/// Run `agent-svgtools` with `args` and return stdout as text.
fn run(args: &[OsString]) -> String {
    let out = bin()
        .args(args)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(out).expect("stdout is UTF-8")
}

/// Make numbers comparable across encoders: TOON writes `93` where JSON
/// writes `93.0`, and both mean the same value.
fn normalize(value: Value) -> Value {
    match value {
        Value::Number(n) => Value::from(n.as_f64().expect("finite number")),
        Value::Array(items) => Value::Array(items.into_iter().map(normalize).collect()),
        Value::Object(map) => {
            Value::Object(map.into_iter().map(|(k, v)| (k, normalize(v))).collect())
        }
        other => other,
    }
}

/// Assert that the TOON output of `args` decodes to the JSON output of `args`.
/// Returns the decoded value so callers can check its shape.
fn assert_round_trip(args: &[OsString]) -> Value {
    let toon = run(args);

    let mut json_args = args.to_vec();
    json_args.extend(["--format".into(), "json".into()]);
    let json: Value = serde_json::from_str(&run(&json_args)).expect("valid JSON");

    let decoded: Value = toon_format::decode_strict(&toon)
        .unwrap_or_else(|e| panic!("TOON output does not decode: {e}\n{toon}"));

    assert_eq!(
        normalize(decoded.clone()),
        normalize(json),
        "TOON output does not decode to the JSON output:\n{toon}"
    );
    decoded
}

fn args(parts: &[&dyn AsRef<std::ffi::OsStr>]) -> Vec<OsString> {
    parts.iter().map(|p| p.as_ref().to_os_string()).collect()
}

#[test]
fn judge_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());
    let diff = dir.path().join("diff.png");
    let decoded = assert_round_trip(&args(&[
        &"judge",
        &reference,
        &fixture("candidate.svg"),
        &"--foreground",
        &"--diff-image",
        &diff,
    ]));

    // The fixture pair has matching dimensions, so `warnings` is empty.
    assert_eq!(decoded["warnings"], Value::Array(vec![]));
    assert!(!decoded["regions"].as_array().unwrap().is_empty());
}

#[test]
fn inspect_round_trips() {
    let decoded = assert_round_trip(&args(&[&"inspect", &fixture("candidate.svg")]));

    // The candidate fixture is a clean vector file, so these lists are empty.
    for key in [
        "prohibited_tags",
        "scripted_attributes",
        "external_references",
    ] {
        assert_eq!(decoded["safety"][key], Value::Array(vec![]), "{key}");
    }
    assert_eq!(decoded["naming"]["ids"], Value::Array(vec![]));
    // Hex colours start with `#` and need quoting.
    assert_eq!(decoded["palette"]["colors"][0], "#1d4ed8");
}

#[test]
fn colors_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());
    assert_round_trip(&args(&[&"colors", &reference]));
}

#[test]
fn compare_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());
    let candidate = dir.path().join("candidate.png");
    bin()
        .arg("render")
        .arg(fixture("candidate.svg"))
        .arg(&candidate)
        .args(["--width", "128"])
        .assert()
        .success();
    assert_round_trip(&args(&[&"compare", &reference, &candidate]));
}
