use crate::project::dom::dom_timeline::dom_shape::DOMShape;
use crate::project::dom::dom_timeline::dom_symbol::DOMSymbolInstance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMGroup {
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Member>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Member {
    #[serde(rename = "DOMShape", default)]
    pub dom_shapes: Vec<DOMShape>,

    #[serde(rename = "DOMSymbolInstance", default)]
    pub dom_symbol_instances: Vec<DOMSymbolInstance>,
}
