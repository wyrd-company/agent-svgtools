//! Schema "snapshot" tests for the machine-readable output formats.
//!
//! Rather than pin brittle byte-for-byte goldens (absolute paths and floating
//! scores vary), these assert the structural contract each format promises: the
//! keys, nesting, and JSONL record shapes agents parse against.

mod common;

use common::{bin, fixture, render_reference};
use serde_json::Value;

fn judge_stdout(format: &str) -> String {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());
    let out = bin()
        .arg("judge")
        .arg(&reference)
        .arg(fixture("candidate.svg"))
        .args(["--format", format])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(out).unwrap()
}

#[test]
fn json_judge_schema() {
    let v: Value = serde_json::from_str(&judge_stdout("json")).expect("valid JSON");

    assert!(v["reference"]["path"].is_string());
    assert!(v["reference"]["width"].is_u64());
    assert!(v["reference"]["height"].is_u64());
    assert!(v["candidate"].is_string());
    assert!(v["rendered"]["width"].is_u64());
    assert!(v["rendered"]["height"].is_u64());

    for key in ["pixel_match", "ssim", "combined"] {
        assert!(v["scores"][key].is_number(), "scores.{key} missing");
    }
    assert!(v["warnings"].is_array());

    let regions = v["regions"].as_array().expect("regions array");
    assert!(!regions.is_empty(), "expected at least one region");
    for r in regions {
        for key in ["x", "y", "width", "height"] {
            assert!(r[key].is_u64(), "region.{key} missing");
        }
        assert!(r["error"].is_number());
    }
}

#[test]
fn jsonl_judge_streams_header_then_regions() {
    let text = judge_stdout("jsonl");
    let lines: Vec<Value> = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("each line is valid JSON"))
        .collect();

    assert!(lines.len() >= 2, "header + at least one region");
    assert_eq!(lines[0]["type"], "judge");
    assert!(lines[0]["scores"]["combined"].is_number());
    assert!(lines[0]["region_count"].is_u64());
    for region in &lines[1..] {
        assert_eq!(region["type"], "region");
        assert!(region["width"].is_u64());
    }
}

#[test]
fn toon_judge_is_default_and_has_fields() {
    // No --format flag: TOON must be the default.
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());
    let out = bin()
        .arg("judge")
        .arg(&reference)
        .arg(fixture("candidate.svg"))
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(out).unwrap();

    // TOON has no braces/quotes-everywhere; assert the key tokens are present.
    for token in ["scores:", "pixel_match:", "ssim:", "combined:", "regions"] {
        assert!(
            text.contains(token),
            "TOON output missing `{token}`:\n{text}"
        );
    }
    assert!(
        !text.trim_start().starts_with('{'),
        "default output looks like JSON, not TOON"
    );
}

#[test]
fn colors_jsonl_tags_each_record() {
    let dir = tempfile::tempdir().unwrap();
    let reference = render_reference(dir.path());
    let out = bin()
        .arg("colors")
        .arg(&reference)
        .args(["--format", "jsonl", "--max-colors", "4"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(out).unwrap();
    let lines: Vec<Value> = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(lines[0]["type"], "summary");
    assert!(lines[1..].iter().all(|l| l["type"] == "color"));
    assert!(lines[1]["hex"].is_string());
}
