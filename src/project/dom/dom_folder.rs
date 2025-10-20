use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMFolders {
    #[serde(rename = "DOMFolderItem", default)]
    pub items: Vec<FolderItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FolderItem {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@itemID", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,

    #[serde(rename = "@isExpanded", skip_serializing_if = "Option::is_none")]
    pub is_expanded: Option<bool>,
}
