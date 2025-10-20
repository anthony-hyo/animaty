use crate::project::dom::dom_timeline::dom_frame::Frames;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Layers {
    #[serde(rename = "DOMLayer", default)]
    pub dom_layers: Vec<Layer>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Layer {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(rename = "@autoNamed", skip_serializing_if = "Option::is_none")]
    pub auto_named: Option<bool>,

    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,

    #[serde(rename = "@current", skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,

    #[serde(rename = "@isSelected", skip_serializing_if = "Option::is_none")]
    pub is_selected: Option<bool>,

    #[serde(rename = "@locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    #[serde(rename = "frames", default)]
    pub frames: Vec<Frames>,
}
