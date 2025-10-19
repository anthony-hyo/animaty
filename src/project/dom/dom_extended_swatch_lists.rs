use crate::project::dom::dom_swatch_lists::SwatchListInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMExtendedSwatchLists {
    #[serde(rename = "swatchList", default)]
    pub swatch_lists: Vec<SwatchList>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SwatchList {
    #[serde(rename = "swatchListInfo")]
    pub swatch_list_info: Option<SwatchListInfo>,

    #[serde(rename = "swatches")]
    pub swatches: Option<SwatchesContainer>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SwatchesContainer {
    #[serde(rename = "$value")]
    pub items: Vec<SwatchesItems>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde()]
pub enum SwatchesItems {
    #[serde(rename = "swatchListInfo")]
    SwatchListInfo(SwatchListInfo),

    #[serde(rename = "LinearGradientSwatchItem")]
    LinearGradient(SwatchItem),

    #[serde(rename = "RadialGradientSwatchItem")]
    RadialGradient(SwatchItem),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SwatchItem {
    #[serde(rename = "GradientEntry", default)]
    pub entries: Vec<GradientEntry>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GradientEntry {
    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@ratio")]
    pub ratio: Option<f64>,

    #[serde(rename = "@alpha")]
    pub alpha: Option<f64>,
}
