use crate::project::dom::dom_timeline::dom_group::DOMGroup;
use crate::project::dom::dom_timeline::dom_shape::DOMShape;
use crate::project::dom::dom_timeline::dom_symbol::DOMSymbolInstance;
use crate::project::dom::dom_timeline::dom_text::DOMStaticText;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Elements {
    #[serde(rename = "$value", default)]
    pub items: Vec<Element>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde()]
pub enum Element {
    #[serde(rename = "DOMShape")]
    DOMShape(DOMShape),

    #[serde(rename = "DOMGroup")]
    DOMGroup(DOMGroup),

    #[serde(rename = "DOMSymbolInstance")]
    DOMSymbolInstance(DOMSymbolInstance),

    #[serde(rename = "DOMStaticText")]
    DOMStaticText(DOMStaticText),
}
