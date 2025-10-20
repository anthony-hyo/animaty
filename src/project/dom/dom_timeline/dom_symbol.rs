use crate::project::dom::dom_timeline::dom_matrix::{Matrices, Points};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMSymbolInstance {
    #[serde(rename = "@libraryItemName", skip_serializing_if = "Option::is_none")]
    pub library_item_name: Option<String>,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@symbolType", skip_serializing_if = "Option::is_none")]
    pub symbol_type: Option<String>,

    #[serde(rename = "@loop", skip_serializing_if = "Option::is_none")]
    pub loop_type: Option<String>,

    #[serde(rename = "@selected", skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,

    #[serde(rename = "@centerPoint3DX", skip_serializing_if = "Option::is_none")]
    pub center_point_3d_x: Option<String>,

    #[serde(rename = "@centerPoint3DY", skip_serializing_if = "Option::is_none")]
    pub center_point_3d_y: Option<String>,

    #[serde(rename = "@accName", skip_serializing_if = "Option::is_none")]
    pub acc_name: Option<String>,

    #[serde(rename = "@description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "@shortcut", skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,

    #[serde(rename = "@tabIndex", skip_serializing_if = "Option::is_none")]
    pub tab_index: Option<u32>,

    #[serde(rename = "@forceSimple", skip_serializing_if = "Option::is_none")]
    pub force_simple: Option<bool>,

    #[serde(rename = "@hasAccessibleData", skip_serializing_if = "Option::is_none")]
    pub has_accessible_data: Option<bool>,

    #[serde(rename = "matrix", skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrices>,

    #[serde(
        rename = "transformationPoint",
        skip_serializing_if = "Option::is_none"
    )]
    pub transformation_point: Option<Points>,
}
