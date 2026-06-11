//! `agent-svgtools colors` — dominant color palette extraction.

use crate::commands::shared::OutputCtx;
use crate::io::load_rgba;
use crate::output::{emit_stdout, Report};
use crate::palette::{extract, PaletteColor};
use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;

/// Extract the dominant color palette of an image as hex values, with coverage
/// and a sample location for each.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Input raster image.
    pub image: PathBuf,
    /// Maximum number of colors to report.
    #[arg(long, default_value_t = 8)]
    pub max_colors: usize,
}

#[derive(Serialize)]
struct ColorsReport {
    image: String,
    colors: Vec<PaletteColor>,
}

impl Report for ColorsReport {
    fn jsonl_records(&self) -> Vec<serde_json::Value> {
        let mut records = vec![serde_json::json!({
            "type": "summary",
            "image": self.image,
            "count": self.colors.len(),
        })];
        for c in &self.colors {
            let mut v = serde_json::to_value(c).unwrap_or(serde_json::Value::Null);
            if let Some(obj) = v.as_object_mut() {
                obj.insert("type".into(), serde_json::Value::from("color"));
            }
            records.push(v);
        }
        records
    }
}

pub fn run(args: &Args, ctx: &OutputCtx) -> Result<i32> {
    let img = load_rgba(&args.image)?;
    let colors = extract(&img, args.max_colors);
    let report = ColorsReport {
        image: args.image.display().to_string(),
        colors,
    };
    emit_stdout(&report, ctx.format, ctx.pretty)?;
    Ok(0)
}
