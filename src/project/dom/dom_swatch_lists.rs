use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMSwatchLists {
    #[serde(rename = "swatchList", default)]
    pub swatch_lists: Vec<SwatchList>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SwatchList {
    #[serde(rename = "swatchListInfo")]
    pub swatch_list_info: Option<SwatchListInfo>,

    #[serde(rename = "swatches")]
    pub swatches: Option<Swatches>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SwatchListInfo {
    #[serde(rename = "@name")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Swatches {
    #[serde(rename = "swatchListInfo")]
    pub swatch_list_info: Option<SwatchListInfo>,

    #[serde(rename = "SolidSwatchItem", default)]
    pub solid_swatch_items: Vec<SolidSwatchItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SolidSwatchItem {
    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@hue")]
    pub hue: Option<u32>,

    #[serde(rename = "@saturation")]
    pub saturation: Option<u32>,

    #[serde(rename = "@brightness")]
    pub brightness: Option<u32>,

    #[serde(rename = "@alpha")]
    pub alpha: Option<f64>,
}
