use crate::project::dom::dom_timeline::dom_matrix::{Matrices, Points};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMSymbolInstance {
    #[serde(rename = "@libraryItemName")]
    pub library_item_name: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@symbolType")]
    pub symbol_type: Option<String>,

    #[serde(rename = "@loop")]
    pub loop_type: Option<String>,

    #[serde(rename = "@selected")]
    pub selected: Option<bool>,

    #[serde(rename = "@centerPoint3DX")]
    pub center_point_3d_x: Option<String>,

    #[serde(rename = "@centerPoint3DY")]
    pub center_point_3d_y: Option<String>,

    #[serde(rename = "@accName")]
    pub acc_name: Option<String>,

    #[serde(rename = "@description")]
    pub description: Option<String>,

    #[serde(rename = "@shortcut")]
    pub shortcut: Option<String>,

    #[serde(rename = "@tabIndex")]
    pub tab_index: Option<u32>,

    #[serde(rename = "@forceSimple")]
    pub force_simple: Option<bool>,

    #[serde(rename = "@hasAccessibleData")]
    pub has_accessible_data: Option<bool>,

    #[serde(rename = "matrix")]
    pub matrix: Option<Matrices>,

    #[serde(rename = "transformationPoint")]
    pub transformation_point: Option<Points>,
}
