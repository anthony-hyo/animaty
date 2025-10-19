use crate::project::dom::dom_timeline::dom_frame::Frames;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Layers {
    #[serde(rename = "DOMLayer", default)]
    pub dom_layers: Vec<Layer>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Layer {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@autoNamed")]
    pub auto_named: Option<bool>,

    #[serde(rename = "@visible")]
    pub visible: Option<bool>,

    #[serde(rename = "@current")]
    pub current: Option<bool>,

    #[serde(rename = "@isSelected")]
    pub is_selected: Option<bool>,

    #[serde(rename = "@locked")]
    pub locked: Option<bool>,

    #[serde(rename = "frames", default)]
    pub frames: Vec<Frames>,
}
