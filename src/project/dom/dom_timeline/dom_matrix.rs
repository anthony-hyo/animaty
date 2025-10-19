use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Matrices {
    #[serde(rename = "Matrix")]
    pub matrix: Matrix,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Matrix {
    #[serde(rename = "@a")]
    pub a: Option<f64>,

    #[serde(rename = "@d")]
    pub d: Option<f64>,

    #[serde(rename = "@tx")]
    pub tx: Option<f64>,

    #[serde(rename = "@ty")]
    pub ty: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Points {
    #[serde(rename = "Point")]
    pub point: Point,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Point {
    #[serde(rename = "@x")]
    pub x: Option<f64>,

    #[serde(rename = "@y")]
    pub y: Option<f64>,
}
