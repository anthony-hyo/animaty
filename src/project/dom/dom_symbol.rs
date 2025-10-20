use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMSymbols {
    #[serde(rename = "Include", default)]
    pub includes: Vec<SymbolInclude>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SymbolInclude {
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,

    #[serde(rename = "@loadImmediate", skip_serializing_if = "Option::is_none")]
    pub load_immediate: Option<bool>,

    #[serde(rename = "@itemID", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,

    #[serde(rename = "@itemIcon", skip_serializing_if = "Option::is_none")]
    pub item_icon: Option<u32>,

    #[serde(rename = "@lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<u32>,
}
