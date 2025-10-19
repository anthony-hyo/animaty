use crate::project::dom::dom_timeline::dom_matrix::Matrices;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMStaticText {
    #[serde(rename = "@fontRenderingMode")]
    pub font_rendering_mode: Option<String>,

    #[serde(rename = "@width")]
    pub width: Option<String>,

    #[serde(rename = "@height")]
    pub height: Option<String>,

    #[serde(rename = "@isSelectable")]
    pub is_selectable: Option<bool>,

    #[serde(rename = "matrix")]
    pub matrix: Option<Matrices>,

    #[serde(rename = "textRuns")]
    pub text_runs: Option<TextRunsContainer>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TextRunsContainer {
    #[serde(rename = "DOMTextRun", default)]
    pub dom_text_runs: Vec<DOMTextRun>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMTextRun {
    #[serde(rename = "characters")]
    pub characters: Option<String>,

    #[serde(rename = "textAttrs")]
    pub text_attrs: Option<TextAttrsContainer>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TextAttrsContainer {
    #[serde(rename = "DOMTextAttrs")]
    pub dom_text_attrs: Option<DOMTextAttrs>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMTextAttrs {
    #[serde(rename = "@aliasText")]
    pub alias_text: Option<bool>,

    #[serde(rename = "@autoKern")]
    pub auto_kern: Option<bool>,

    #[serde(rename = "@face")]
    pub face: Option<String>,
}
