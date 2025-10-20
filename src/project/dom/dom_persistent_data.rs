use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMPersistentData {
    #[serde(rename = "PD", default)]
    pub items: Vec<PersistentDataItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PersistentDataItem {
    #[serde(rename = "@n", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@v", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
