use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMTimelines {
    #[serde(rename = "DOMTimeline", default)]
    pub timelines: Vec<Timeline>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Timeline {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "layers", default)]
    pub layers: Vec<Layer>,
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

    #[serde(rename = "frames", default)]
    pub frames: Vec<Frame>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Frame {
    #[serde(rename = "@index")]
    pub index: Option<u32>,

    #[serde(rename = "@duration")]
    pub duration: Option<u32>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@labelType")]
    pub label_type: Option<String>,

    #[serde(rename = "@keyMode")]
    pub key_mode: Option<u32>,

    #[serde(rename = "elements", default)]
    pub elements: Vec<Element>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Element {
    #[serde(rename = "@libraryItemName")]
    pub library_item_name: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@symbolType")]
    pub symbol_type: Option<String>,

    #[serde(rename = "@loop")]
    pub r#loop: Option<String>,

    #[serde(rename = "@selected")]
    pub selected: Option<bool>,

    #[serde(rename = "matrix")]
    pub matrix: Option<DOMMatrix>,

    #[serde(rename = "transformationPoint")]
    pub transformation_point: Option<DOMPoint>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMMatrix {
    #[serde(rename = "Matrix")]
    pub matrix: Option<Matrix>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Matrix {
    #[serde(rename = "@a")]
    pub a: Option<f64>,

    #[serde(rename = "@b")]
    pub b: Option<f64>,

    #[serde(rename = "@c")]
    pub c: Option<f64>,

    #[serde(rename = "@d")]
    pub d: Option<f64>,

    #[serde(rename = "@tx")]
    pub tx: Option<f64>,

    #[serde(rename = "@ty")]
    pub ty: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMPoint {
    #[serde(rename = "Point")]
    pub point: Option<Point>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Point {
    #[serde(rename = "@x")]
    pub x: Option<f64>,

    #[serde(rename = "@y")]
    pub y: Option<f64>,
}
