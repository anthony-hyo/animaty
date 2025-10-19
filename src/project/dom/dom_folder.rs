use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMFolders {
    #[serde(rename = "DOMFolderItem", default)]
    pub items: Vec<FolderItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FolderItem {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@itemID")]
    pub item_id: Option<String>,

    #[serde(rename = "@isExpanded")]
    pub is_expanded: Option<bool>,
}
