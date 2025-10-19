use crate::project::dom::dom_timeline::dom_layer::Layers;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMTimelines {
    #[serde(rename = "DOMTimeline", default)]
    pub dom_timelines: Vec<Timeline>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Timeline {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@currentFrame")]
    pub current_frame: Option<u32>,

    #[serde(rename = "layers", default)]
    pub layers: Vec<Layers>,
}
