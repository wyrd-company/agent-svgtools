//! The `judge` report schema and its JSONL projection.

use crate::commands::shared::ArtifactPaths;
use crate::compare::{Comparison, Region};
use serde::Serialize;

/// Reference image metadata.
#[derive(Serialize)]
pub struct ReferenceInfo {
    pub path: String,
    pub width: u32,
    pub height: u32,
}

/// Pixel dimensions.
#[derive(Serialize)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

/// The three headline scores.
#[derive(Serialize)]
pub struct Scores {
    pub scope: &'static str,
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

/// The full `judge` result.
#[derive(Serialize)]
pub struct JudgeReport {
    pub reference: ReferenceInfo,
    pub candidate: String,
    pub rendered: Dimensions,
    pub scores: Scores,
    pub warnings: Vec<String>,
    pub regions: Vec<Region>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<ArtifactPaths>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<ThresholdResult>,
}

impl JudgeReport {
    /// Assemble a report from its parts.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        reference_path: String,
        reference_dims: (u32, u32),
        candidate_path: String,
        comparison: Comparison,
        warnings: Vec<String>,
        artifacts: Option<ArtifactPaths>,
        threshold: Option<ThresholdResult>,
    ) -> Self {
        Self {
            reference: ReferenceInfo {
                path: reference_path,
                width: reference_dims.0,
                height: reference_dims.1,
            },
            candidate: candidate_path,
            rendered: Dimensions {
                width: comparison.width,
                height: comparison.height,
            },
            scores: Scores {
                scope: comparison.score_scope,
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

impl crate::output::Report for JudgeReport {
    fn jsonl_records(&self) -> Vec<serde_json::Value> {
        // Header record (everything except the region list)...
        let mut header = serde_json::json!({
            "type": "judge",
            "reference": self.reference,
            "candidate": self.candidate,
            "rendered": self.rendered,
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
        // ...then one record per region.
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
