use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Matrices {
    #[serde(rename = "Matrix")]
    pub matrix: Matrix,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Matrix {
    #[serde(rename = "@a", skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,

    #[serde(rename = "@d", skip_serializing_if = "Option::is_none")]
    pub d: Option<f64>,

    #[serde(rename = "@tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<f64>,

    #[serde(rename = "@ty", skip_serializing_if = "Option::is_none")]
    pub ty: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Points {
    #[serde(rename = "Point")]
    pub point: Point,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Point {
    #[serde(rename = "@x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,

    #[serde(rename = "@y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}
