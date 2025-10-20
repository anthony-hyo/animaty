use crate::project::dom::dom_timeline::dom_matrix::Matrices;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMShape {
    #[serde(rename = "@isFloating", skip_serializing_if = "Option::is_none")]
    pub is_floating: Option<bool>,

    #[serde(rename = "@selected", skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,

    #[serde(rename = "matrix", skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrices>,

    #[serde(rename = "fills", skip_serializing_if = "Option::is_none")]
    pub fills: Option<FillsContainer>,

    #[serde(rename = "strokes", skip_serializing_if = "Option::is_none")]
    pub strokes: Option<StrokesContainer>,

    #[serde(rename = "edges", skip_serializing_if = "Option::is_none")]
    pub edges: Option<EdgesContainer>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StrokesContainer {
    #[serde(rename = "StrokeStyle", default)]
    pub stroke_styles: Vec<StrokeStyle>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StrokeStyle {
    #[serde(rename = "@index", skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,

    #[serde(rename = "SolidStroke", skip_serializing_if = "Option::is_none")]
    pub solid_stroke: Option<SolidStroke>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SolidStroke {
    #[serde(rename = "@scaleMode", skip_serializing_if = "Option::is_none")]
    pub scale_mode: Option<String>,

    #[serde(rename = "fill", skip_serializing_if = "Option::is_none")]
    pub fill: Option<SolidStrokeFill>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SolidStrokeFill {
    #[serde(rename = "SolidColor", skip_serializing_if = "Option::is_none")]
    pub solid_color: Option<SolidColor>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SolidColor {
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EdgesContainer {
    #[serde(rename = "Edge", default)]
    pub edges: Vec<Edge>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Edge {
    #[serde(rename = "@fillStyle0", skip_serializing_if = "Option::is_none")]
    pub fill_style_0: Option<String>,

    #[serde(rename = "@fillStyle1", skip_serializing_if = "Option::is_none")]
    pub fill_style_1: Option<String>,

    #[serde(rename = "@strokeStyle", skip_serializing_if = "Option::is_none")]
    pub stroke_style: Option<String>,

    #[serde(rename = "@edges", skip_serializing_if = "Option::is_none")]
    pub edges: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FillsContainer {
    #[serde(rename = "FillStyle", default)]
    pub fill_styles: Vec<FillStyle>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FillStyle {
    #[serde(rename = "@index", skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,

    #[serde(rename = "SolidColor", skip_serializing_if = "Option::is_none")]
    pub solid_color: Option<SolidColor>,
}
