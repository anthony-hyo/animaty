use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMSymbols {
    #[serde(rename = "Include", default)]
    pub includes: Vec<SymbolInclude>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SymbolInclude {
    #[serde(rename = "@href")]
    pub href: Option<String>,

    #[serde(rename = "@loadImmediate")]
    pub load_immediate: Option<bool>,

    #[serde(rename = "@itemID")]
    pub item_id: Option<String>,

    #[serde(rename = "@itemIcon")]
    pub item_icon: Option<u32>,

    #[serde(rename = "@lastModified")]
    pub last_modified: Option<u32>,
}
