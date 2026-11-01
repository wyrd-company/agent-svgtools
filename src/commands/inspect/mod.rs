// ---
// relationships:
//   implements: README
// ---
//! `agent-svgtools inspect` — deterministic SVG structure and safety audit.

use crate::commands::shared::OutputCtx;
use crate::output::{emit_stdout, Report};
use anyhow::{Context, Result};
use roxmltree::{Document, Node};
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

const XLINK_NS: &str = "http://www.w3.org/1999/xlink";

const GRAPHIC_TAGS: &[&str] = &[
    "circle", "ellipse", "line", "path", "polygon", "polyline", "rect", "text", "use",
];
const PROHIBITED_TAGS: &[&str] = &["foreignObject", "image", "script"];

/// Inspect an SVG's structure, authored palette, naming, and external-resource
/// safety without rasterizing it.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// SVG file to inspect.
    pub svg: PathBuf,
}

#[derive(Serialize)]
struct Structure {
    authored_graphics: usize,
    authored_paths: usize,
    path_commands: usize,
    groups: usize,
    reusable_definitions: usize,
}

#[derive(Serialize)]
struct Naming {
    named_components: usize,
    ids: Vec<String>,
}

#[derive(Serialize)]
struct Palette {
    count: usize,
    colors: Vec<String>,
}

#[derive(Serialize)]
struct Safety {
    vector_only: bool,
    prohibited_tags: Vec<String>,
    scripted_attributes: Vec<String>,
    external_references: Vec<String>,
    embedded_data: bool,
}

#[derive(Serialize)]
struct InspectReport {
    svg: String,
    view_box: Option<String>,
    width: Option<String>,
    height: Option<String>,
    structure: Structure,
    naming: Naming,
    palette: Palette,
    safety: Safety,
}

impl Report for InspectReport {}

pub fn run(args: &Args, ctx: &OutputCtx) -> Result<i32> {
    let text = fs::read_to_string(&args.svg)
        .with_context(|| format!("failed to read SVG {}", args.svg.display()))?;
    let document = Document::parse(&text)
        .with_context(|| format!("failed to parse SVG {}", args.svg.display()))?;
    let root = document.root_element();

    let elements: Vec<_> = document.descendants().filter(Node::is_element).collect();
    let authored_graphics = elements
        .iter()
        .filter(|node| is_authored_graphic(**node))
        .count();
    let authored_paths: Vec<_> = elements
        .iter()
        .filter(|node| node.tag_name().name() == "path")
        .collect();
    let path_commands = authored_paths
        .iter()
        .map(|node| count_path_commands(svg_attribute(**node, "d").unwrap_or_default()))
        .sum();

    let referenced_ids: BTreeSet<_> = elements
        .iter()
        .filter_map(|node| href(node))
        .filter_map(|value| value.strip_prefix('#'))
        .map(str::to_owned)
        .collect();
    let ids: BTreeSet<_> = elements
        .iter()
        .filter(|node| {
            let tag = node.tag_name().name();
            (tag == "g" || GRAPHIC_TAGS.contains(&tag))
                && (!has_ancestor(**node, "defs")
                    || svg_attribute(**node, "id").is_some_and(|id| referenced_ids.contains(id)))
        })
        .filter_map(|node| svg_attribute(*node, "id"))
        .filter(|id| !id.trim().is_empty())
        .map(str::to_owned)
        .collect();

    let colors: BTreeSet<_> = elements
        .iter()
        .flat_map(|node| presentation_colors(*node))
        .chain(
            elements
                .iter()
                .filter(|node| node.tag_name().name() == "style")
                .filter_map(|node| node.text())
                .flat_map(stylesheet_colors),
        )
        .collect();
    let prohibited_tags: BTreeSet<_> = elements
        .iter()
        .map(|node| node.tag_name().name())
        .filter(|tag| PROHIBITED_TAGS.contains(tag))
        .map(str::to_owned)
        .collect();
    let scripted_attributes: BTreeSet<_> = elements
        .iter()
        .flat_map(|node| node.attributes())
        .filter(|attribute| attribute.namespace().is_none())
        .map(|attribute| attribute.name().to_ascii_lowercase())
        .filter(|name| name.starts_with("on"))
        .collect();
    let external_references: BTreeSet<_> = elements
        .iter()
        .flat_map(|node| node.attributes())
        .filter_map(|attribute| {
            let value = attribute.value().trim();
            // Keep scanning all values for URLs, but only SVG/XLink href
            // attributes carry implicit external-reference semantics.
            let is_href = attribute.name() == "href"
                && matches!(attribute.namespace(), None | Some(XLINK_NS));
            (is_external(value) || (is_href && !value.starts_with('#'))).then(|| value.to_owned())
        })
        .chain(
            elements
                .iter()
                .filter(|node| node.tag_name().name() == "style")
                .filter_map(|node| node.text())
                .map(str::trim)
                .filter(|value| {
                    is_external(value) || value.to_ascii_lowercase().contains("@import")
                })
                .map(str::to_owned),
        )
        .collect();
    let embedded_data = text.to_ascii_lowercase().contains("data:");
    let vector_only = prohibited_tags.is_empty()
        && scripted_attributes.is_empty()
        && external_references.is_empty()
        && !embedded_data;

    let report = InspectReport {
        svg: args.svg.display().to_string(),
        view_box: svg_attribute(root, "viewBox").map(str::to_owned),
        width: svg_attribute(root, "width").map(str::to_owned),
        height: svg_attribute(root, "height").map(str::to_owned),
        structure: Structure {
            authored_graphics,
            authored_paths: authored_paths.len(),
            path_commands,
            groups: elements
                .iter()
                .filter(|node| node.tag_name().name() == "g")
                .count(),
            reusable_definitions: referenced_ids.len(),
        },
        naming: Naming {
            named_components: ids.len(),
            ids: ids.into_iter().collect(),
        },
        palette: Palette {
            count: colors.len(),
            colors: colors.into_iter().collect(),
        },
        safety: Safety {
            vector_only,
            prohibited_tags: prohibited_tags.into_iter().collect(),
            scripted_attributes: scripted_attributes.into_iter().collect(),
            external_references: external_references.into_iter().collect(),
            embedded_data,
        },
    };
    emit_stdout(&report, ctx.format, ctx.pretty)?;
    Ok(0)
}

fn has_ancestor(node: Node<'_, '_>, name: &str) -> bool {
    node.ancestors()
        .skip(1)
        .any(|ancestor| ancestor.is_element() && ancestor.tag_name().name() == name)
}

fn is_authored_graphic(node: Node<'_, '_>) -> bool {
    if !GRAPHIC_TAGS.contains(&node.tag_name().name()) {
        return false;
    }
    if node.ancestors().any(|ancestor| {
        ancestor.is_element()
            && matches!(
                ancestor.tag_name().name(),
                "defs" | "clipPath" | "mask" | "marker" | "pattern" | "symbol"
            )
    }) {
        return false;
    }
    if node.ancestors().filter(Node::is_element).any(|ancestor| {
        matches!(
            presentation_value(ancestor, "display").as_deref(),
            Some("none")
        ) || matches!(
            presentation_value(ancestor, "visibility").as_deref(),
            Some("hidden" | "collapse")
        ) || presentation_value(ancestor, "opacity")
            .and_then(|value| value.parse::<f64>().ok())
            .is_some_and(|opacity| opacity <= 0.0)
    }) {
        return false;
    }
    !(matches!(presentation_value(node, "fill").as_deref(), Some("none"))
        && matches!(presentation_value(node, "stroke").as_deref(), Some("none")))
}

fn presentation_value(node: Node<'_, '_>, name: &str) -> Option<String> {
    svg_attribute(node, name)
        .map(|value| value.trim().to_ascii_lowercase())
        .or_else(|| {
            svg_attribute(node, "style")?
                .split(';')
                .find_map(|declaration| {
                    let (property, value) = declaration.split_once(':')?;
                    (property.trim().eq_ignore_ascii_case(name))
                        .then(|| value.trim().to_ascii_lowercase())
                })
        })
}

// Unprefixed SVG attributes have no namespace, including under a default SVG
// namespace. roxmltree 0.21's string lookup matches any namespace instead.
fn svg_attribute<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    node.attributes()
        .find(|attribute| attribute.namespace().is_none() && attribute.name() == name)
        .map(|attribute| attribute.value())
}

fn href<'a>(node: &Node<'a, '_>) -> Option<&'a str> {
    svg_attribute(*node, "href").or_else(|| node.attribute((XLINK_NS, "href")))
}

fn count_path_commands(data: &str) -> usize {
    data.bytes()
        .filter(|byte| {
            matches!(
                byte.to_ascii_uppercase(),
                b'A' | b'C' | b'H' | b'L' | b'M' | b'Q' | b'S' | b'T' | b'V' | b'Z'
            )
        })
        .count()
}

fn presentation_colors(node: Node<'_, '_>) -> Vec<String> {
    let mut values = ["fill", "stroke", "stop-color"]
        .into_iter()
        .filter_map(|name| svg_attribute(node, name))
        .filter_map(normalize_color)
        .collect::<Vec<_>>();
    if let Some(style) = svg_attribute(node, "style") {
        values.extend(style.split(';').filter_map(|declaration| {
            let (name, value) = declaration.split_once(':')?;
            matches!(name.trim(), "fill" | "stroke" | "stop-color")
                .then(|| normalize_color(value))
                .flatten()
        }));
    }
    values
}

fn stylesheet_colors(css: &str) -> Vec<String> {
    css.split(['{', '}', ';'])
        .filter_map(|declaration| {
            let (name, value) = declaration.split_once(':')?;
            matches!(name.trim(), "fill" | "stroke" | "stop-color")
                .then(|| normalize_color(value))
                .flatten()
        })
        .collect()
}

fn normalize_color(value: &str) -> Option<String> {
    let value = value.trim().to_ascii_lowercase();
    if value.is_empty()
        || matches!(value.as_str(), "none" | "currentcolor" | "transparent")
        || value.starts_with("url(")
    {
        return None;
    }
    if let Some(hex) = value.strip_prefix('#') {
        return match hex.len() {
            3 | 4 => Some(format!(
                "#{}",
                hex.chars()
                    .flat_map(|character| [character, character])
                    .collect::<String>()
            )),
            6 | 8 => Some(format!("#{hex}")),
            _ => Some(value),
        };
    }
    Some(value)
}

fn is_external(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    if ["data:", "http:", "https:", "file:", "javascript:"]
        .iter()
        .any(|prefix| value.contains(prefix))
    {
        return true;
    }
    value.match_indices("url(").any(|(start, _)| {
        value[start + 4..]
            .split_once(')')
            .map(|(target, _)| {
                target.trim_matches(|c: char| c.is_whitespace() || c == '\'' || c == '"')
            })
            .is_some_and(|target| !target.starts_with('#'))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_short_hex_colors() {
        assert_eq!(normalize_color("#AbC"), Some("#aabbcc".into()));
        assert_eq!(normalize_color("#1234"), Some("#11223344".into()));
    }

    #[test]
    fn counts_only_svg_path_commands() {
        assert_eq!(count_path_commands("M0 0 C1e2 3 4 5 6 7z"), 3);
    }

    #[test]
    fn distinguishes_local_paint_servers_from_external_urls() {
        assert!(!is_external("url(#wing-gradient)"));
        assert!(is_external("url(assets.svg#wing-gradient)"));
        assert!(is_external("@import url('theme.css')"));
    }

    #[test]
    fn reads_colors_from_stylesheet_declarations() {
        assert_eq!(
            stylesheet_colors(".part { fill: #f00; stroke: #001122; }"),
            vec!["#ff0000", "#001122"]
        );
    }

    #[test]
    fn excludes_common_non_rendering_graphics() {
        let document = Document::parse(
            r#"<svg xmlns="http://www.w3.org/2000/svg">
                <rect id="shown"/>
                <g display="none"><circle/></g>
                <path fill="none" stroke="none"/>
                <ellipse style="visibility: hidden"/>
                <defs><polygon/></defs>
              </svg>"#,
        )
        .unwrap();
        assert_eq!(
            document
                .descendants()
                .filter(Node::is_element)
                .filter(|node| is_authored_graphic(*node))
                .count(),
            1
        );
    }
}
