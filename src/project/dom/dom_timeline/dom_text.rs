use crate::project::dom::dom_timeline::dom_matrix::Matrices;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMStaticText {
    #[serde(rename = "@fontRenderingMode", skip_serializing_if = "Option::is_none")]
    pub font_rendering_mode: Option<String>,

    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,

    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,

    #[serde(rename = "@isSelectable", skip_serializing_if = "Option::is_none")]
    pub is_selectable: Option<bool>,

    #[serde(rename = "matrix", skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrices>,

    #[serde(rename = "textRuns", skip_serializing_if = "Option::is_none")]
    pub text_runs: Option<TextRunsContainer>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TextRunsContainer {
    #[serde(rename = "DOMTextRun", default)]
    pub dom_text_runs: Vec<DOMTextRun>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMTextRun {
    #[serde(rename = "characters", skip_serializing_if = "Option::is_none")]
    pub characters: Option<String>,

    #[serde(rename = "textAttrs", skip_serializing_if = "Option::is_none")]
    pub text_attrs: Option<TextAttrsContainer>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TextAttrsContainer {
    #[serde(rename = "DOMTextAttrs", skip_serializing_if = "Option::is_none")]
    pub dom_text_attrs: Option<DOMTextAttrs>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMTextAttrs {
    #[serde(rename = "@aliasText", skip_serializing_if = "Option::is_none")]
    pub alias_text: Option<bool>,

    #[serde(rename = "@autoKern", skip_serializing_if = "Option::is_none")]
    pub auto_kern: Option<bool>,

    #[serde(rename = "@face", skip_serializing_if = "Option::is_none")]
    pub face: Option<String>,
}
