// ---
// relationships:
//   implements: README
// ---
//! Namespace regressions for SVG inspection.

use serde_json::{json, Value};

fn inspect(svg: &str) -> Value {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.svg");
    std::fs::write(&input, svg).unwrap();
    let output = assert_cmd::Command::cargo_bin("agent-svgtools")
        .unwrap()
        .arg("inspect")
        .arg(input)
        .args(["--format", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&output).unwrap()
}

#[test]
fn inspect_preserves_path_palette_and_reference_semantics() {
    let report = inspect(include_str!("fixtures/namespace-collisions.svg"));
    assert_eq!(report["structure"]["path_commands"], 15);
    assert_eq!(report["naming"]["ids"], json!(["second"]));
    assert_eq!(report["palette"]["colors"], json!(["#0000ff"]));
}

#[test]
fn inspect_ignores_metadata_attribute_names() {
    for reverse in [false, true] {
        let metadata = r##"custom:width="999" custom:height="999" custom:viewBox="wrong""##;
        let svg_attrs = r##"width="32" height="24" viewBox="0 0 32 24""##;
        let root_attrs = if reverse {
            format!("{svg_attrs} {metadata}")
        } else {
            format!("{metadata} {svg_attrs}")
        };
        let report = inspect(&format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:custom="urn:example:metadata" {root_attrs}>
              <path custom:id="wrong" id="shape" custom:d="M0 0" d="M0 0L1 1Z"
                custom:fill="#f00" fill="#00f" custom:stroke="#f00" stroke="#0f0"
                custom:stop-color="#f00" stop-color="#fff" custom:display="none"
                custom:visibility="hidden" custom:opacity="0" custom:style="fill:#f00"/>
              <rect custom:id="metadata-only" custom:d="M0 0" custom:fill="#f00"
                custom:stroke="#f00" custom:stop-color="#f00" custom:style="display:none;fill:#f00"
                custom:onload="metadata" custom:href="metadata.svg"/>
              <circle custom:style="fill:#f00" style="fill:#000"/>
            </svg>"##
        ));
        assert_eq!(report["width"], "32");
        assert_eq!(report["height"], "24");
        assert_eq!(report["view_box"], "0 0 32 24");
        assert_eq!(report["structure"]["path_commands"], 3);
        assert_eq!(report["structure"]["authored_graphics"], 3);
        assert_eq!(report["naming"]["ids"], json!(["shape"]));
        assert_eq!(
            report["palette"]["colors"],
            json!(["#000000", "#0000ff", "#00ff00", "#ffffff"])
        );
        assert_eq!(report["safety"]["scripted_attributes"], json!([]));
        assert_eq!(report["safety"]["external_references"], json!([]));
    }
    let report = inspect(
        r#"<svg xmlns:custom="urn:example:metadata" custom:width="32" custom:height="32" custom:viewBox="0 0 32 32"/>"#,
    );
    assert_eq!(report["width"], Value::Null);
    assert_eq!(report["height"], Value::Null);
    assert_eq!(report["view_box"], Value::Null);
}

#[test]
fn inspect_resolves_href_by_namespace_and_svg_precedence() {
    for (attributes, expected) in [
        (r##"xlink:href="#first" href="#second""##, json!(["second"])),
        (r##"href="#second" xlink:href="#first""##, json!(["second"])),
        (r##"xlink:href="#first""##, json!(["first"])),
        (r##"href="#second""##, json!(["second"])),
        (r##"custom:href="#first""##, json!([])),
        (
            r##"custom:href="#first" href="#second""##,
            json!(["second"]),
        ),
        (
            r##"custom:href="#second" link:href="#first""##,
            json!(["first"]),
        ),
        (r##"xlink:href="#first" href="""##, json!([])),
    ] {
        let report = inspect(&format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:custom="urn:example:metadata"
                xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:link="http://www.w3.org/1999/xlink">
              <defs><path id="first"/><path id="second"/></defs><use {attributes}/>
            </svg>"##
        ));
        assert_eq!(report["naming"]["ids"], expected, "{attributes}");
    }
}
