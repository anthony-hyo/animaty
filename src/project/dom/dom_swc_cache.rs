use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMSwcCache {
    #[serde(rename = "Swc", default)]
    pub swcs: Vec<DOMSwc>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMSwc {
    #[serde(rename = "@hashKey")]
    pub hash_key: Option<String>,

    #[serde(rename = "@href")]
    pub href: Option<String>,

    #[serde(rename = "classDefinitions")]
    pub class_definitions: Option<DOMClassDefinitions>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMClassDefinitions {
    #[serde(rename = "ClassDefinition", default)]
    pub definitions: Vec<DOMClassDefinition>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMClassDefinition {
    #[serde(rename = "@value")]
    pub value: Option<String>,
}
