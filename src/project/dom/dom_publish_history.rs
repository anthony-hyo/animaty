use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMPublishHistory {
    #[serde(rename = "PublishItem", default)]
    pub items: Vec<PublishItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PublishItem {
    #[serde(rename = "@publishSize", skip_serializing_if = "Option::is_none")]
    pub publish_size: Option<u32>,

    #[serde(rename = "@publishTime", skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<u32>,
}
