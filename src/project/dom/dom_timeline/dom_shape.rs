use crate::project::dom::dom_timeline::dom_matrix::Matrices;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMShape {
    #[serde(rename = "@isFloating")]
    pub is_floating: Option<bool>,

    #[serde(rename = "@selected")]
    pub selected: Option<bool>,

    #[serde(rename = "matrix")]
    pub matrix: Option<Matrices>,

    #[serde(rename = "fills")]
    pub fills: Option<FillsContainer>,

    #[serde(rename = "strokes")]
    pub strokes: Option<StrokesContainer>,

    #[serde(rename = "edges")]
    pub edges: Option<EdgesContainer>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StrokesContainer {
    #[serde(rename = "StrokeStyle", default)]
    pub stroke_styles: Vec<StrokeStyle>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StrokeStyle {
    #[serde(rename = "@index")]
    pub index: Option<u32>,

    #[serde(rename = "SolidStroke")]
    pub solid_stroke: Option<SolidStroke>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SolidStroke {
    #[serde(rename = "@scaleMode")]
    pub scale_mode: Option<String>,

    #[serde(rename = "fill")]
    pub fill: Option<SolidStrokeFill>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SolidStrokeFill {
    #[serde(rename = "SolidColor")]
    pub solid_color: Option<SolidColor>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SolidColor {
    #[serde(rename = "@color")]
    pub color: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct EdgesContainer {
    #[serde(rename = "Edge", default)]
    pub edges: Vec<Edge>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Edge {
    #[serde(rename = "@fillStyle0")]
    pub fill_style_0: Option<String>,

    #[serde(rename = "@fillStyle1")]
    pub fill_style_1: Option<String>,

    #[serde(rename = "@strokeStyle")]
    pub stroke_style: Option<String>,

    #[serde(rename = "@edges")]
    pub edges: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FillsContainer {
    #[serde(rename = "FillStyle", default)]
    pub fill_styles: Vec<FillStyle>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FillStyle {
    #[serde(rename = "@index")]
    pub index: Option<u32>,

    #[serde(rename = "SolidColor")]
    pub solid_color: Option<SolidColor>,
}
