use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphShape {
    #[serde(rename = "morphSegments")]
    pub morph_segments: Option<MorphSegmentsContainer>,

    #[serde(rename = "morphHintsList")]
    pub morph_hints_list: Option<MorphHintsList>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphSegmentsContainer {
    #[serde(rename = "MorphSegment", default)]
    pub morph_segments: Vec<MorphSegment>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphSegment {
    #[serde(rename = "@startPointB")]
    pub start_point_b: Option<String>,

    #[serde(rename = "@strokeIndex1")]
    pub stroke_index1: Option<String>,

    #[serde(rename = "@strokeIndex2")]
    pub stroke_index2: Option<String>,

    #[serde(rename = "@fillIndex1")]
    pub fill_index1: Option<String>,

    #[serde(rename = "@fillIndex2")]
    pub fill_index2: Option<String>,

    #[serde(rename = "MorphCurves", default)]
    pub morph_curves: Vec<MorphCurves>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphCurves {
    #[serde(rename = "@controlPointA")]
    pub control_point_a: Option<String>,

    #[serde(rename = "@anchorPointA")]
    pub anchor_point_a: Option<String>,

    #[serde(rename = "@controlPointB")]
    pub control_point_b: Option<String>,

    #[serde(rename = "@anchorPointB")]
    pub anchor_point_b: Option<String>,

    #[serde(rename = "@isLine")]
    pub is_line: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MorphHintsList {}
