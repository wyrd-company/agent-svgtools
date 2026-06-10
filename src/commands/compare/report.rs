//! The `compare` report schema and its JSONL projection.

use crate::commands::shared::ArtifactPaths;
use crate::compare::{Comparison, Region};
use serde::Serialize;

/// The three headline scores.
#[derive(Serialize)]
pub struct Scores {
    pub pixel_match: f64,
    pub ssim: f64,
    pub combined: f64,
}

/// Outcome of a `--threshold` check.
#[derive(Serialize)]
pub struct ThresholdResult {
    pub value: f64,
    pub passed: bool,
}

/// Pixel dimensions the comparison ran at.
#[derive(Serialize)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

/// The full `compare` result.
#[derive(Serialize)]
pub struct CompareReport {
    pub image_a: String,
    pub image_b: String,
    pub dimensions: Dimensions,
    pub scores: Scores,
    pub warnings: Vec<String>,
    pub regions: Vec<Region>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<ArtifactPaths>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<ThresholdResult>,
}

impl CompareReport {
    pub fn new(
        image_a: String,
        image_b: String,
        comparison: Comparison,
        warnings: Vec<String>,
        artifacts: Option<ArtifactPaths>,
        threshold: Option<ThresholdResult>,
    ) -> Self {
        Self {
            image_a,
            image_b,
            dimensions: Dimensions {
                width: comparison.width,
                height: comparison.height,
            },
            scores: Scores {
                pixel_match: comparison.pixel_match,
                ssim: comparison.ssim,
                combined: comparison.combined,
            },
            warnings,
            regions: comparison.regions,
            artifacts,
            threshold,
        }
    }
}

impl crate::output::Report for CompareReport {
    fn jsonl_records(&self) -> Vec<serde_json::Value> {
        let mut header = serde_json::json!({
            "type": "compare",
            "image_a": self.image_a,
            "image_b": self.image_b,
            "dimensions": self.dimensions,
            "scores": self.scores,
            "warnings": self.warnings,
            "region_count": self.regions.len(),
        });
        if let Some(obj) = header.as_object_mut() {
            if let Some(a) = &self.artifacts {
                obj.insert("artifacts".into(), serde_json::to_value(a).unwrap());
            }
            if let Some(t) = &self.threshold {
                obj.insert("threshold".into(), serde_json::to_value(t).unwrap());
            }
        }
        let mut records = vec![header];
        for r in &self.regions {
            let mut v = serde_json::to_value(r).unwrap_or(serde_json::Value::Null);
            if let Some(obj) = v.as_object_mut() {
                obj.insert("type".into(), serde_json::Value::from("region"));
            }
            records.push(v);
        }
        records
    }
}
