use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMPersistentData {
    #[serde(rename = "PD", default)]
    pub data: Vec<PersistentDataItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PersistentDataItem {
    #[serde(rename = "@n")]
    pub name: Option<String>,

    #[serde(rename = "@v")]
    pub value: Option<String>,
}
