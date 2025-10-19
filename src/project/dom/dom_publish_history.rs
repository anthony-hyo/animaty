use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMPublishHistory {
    #[serde(rename = "PublishItem", default)]
    pub items: Vec<PublishItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PublishItem {
    #[serde(rename = "@publishSize")]
    pub publish_size: Option<u32>,

    #[serde(rename = "@publishTime")]
    pub publish_time: Option<u32>,
}
