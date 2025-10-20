use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphShape {
    #[serde(rename = "morphSegments", skip_serializing_if = "Option::is_none")]
    pub morph_segments: Option<MorphSegmentsContainer>,

    #[serde(rename = "morphHintsList", skip_serializing_if = "Option::is_none")]
    pub morph_hints_list: Option<MorphHintsList>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphSegmentsContainer {
    #[serde(rename = "MorphSegment", default)]
    pub morph_segments: Vec<MorphSegment>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphSegment {
    #[serde(rename = "@startPointB", skip_serializing_if = "Option::is_none")]
    pub start_point_b: Option<String>,

    #[serde(rename = "@strokeIndex1", skip_serializing_if = "Option::is_none")]
    pub stroke_index1: Option<String>,

    #[serde(rename = "@strokeIndex2", skip_serializing_if = "Option::is_none")]
    pub stroke_index2: Option<String>,

    #[serde(rename = "@fillIndex1", skip_serializing_if = "Option::is_none")]
    pub fill_index1: Option<String>,

    #[serde(rename = "@fillIndex2", skip_serializing_if = "Option::is_none")]
    pub fill_index2: Option<String>,

    #[serde(rename = "MorphCurves", default)]
    pub morph_curves: Vec<MorphCurves>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphCurves {
    #[serde(rename = "@controlPointA", skip_serializing_if = "Option::is_none")]
    pub control_point_a: Option<String>,

    #[serde(rename = "@anchorPointA", skip_serializing_if = "Option::is_none")]
    pub anchor_point_a: Option<String>,

    #[serde(rename = "@controlPointB", skip_serializing_if = "Option::is_none")]
    pub control_point_b: Option<String>,

    #[serde(rename = "@anchorPointB", skip_serializing_if = "Option::is_none")]
    pub anchor_point_b: Option<String>,

    #[serde(rename = "@isLine", skip_serializing_if = "Option::is_none")]
    pub is_line: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphHintsList {}
