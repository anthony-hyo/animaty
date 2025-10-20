use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMSwatchLists {
    #[serde(rename = "swatchList", default)]
    pub swatch_lists: Vec<SwatchList>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SwatchList {
    #[serde(rename = "swatchListInfo", skip_serializing_if = "Option::is_none")]
    pub swatch_list_info: Option<SwatchListInfo>,

    #[serde(rename = "swatches", skip_serializing_if = "Option::is_none")]
    pub swatches: Option<Swatches>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SwatchListInfo {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Swatches {
    #[serde(rename = "swatchListInfo", skip_serializing_if = "Option::is_none")]
    pub swatch_list_info: Option<SwatchListInfo>,

    #[serde(rename = "SolidSwatchItem", default)]
    pub solid_swatch_items: Vec<SolidSwatchItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SolidSwatchItem {
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(rename = "@hue", skip_serializing_if = "Option::is_none")]
    pub hue: Option<u32>,

    #[serde(rename = "@saturation", skip_serializing_if = "Option::is_none")]
    pub saturation: Option<u32>,

    #[serde(rename = "@brightness", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u32>,

    #[serde(rename = "@alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
}
