use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMFonts {
    #[serde(rename = "DOMFontItem", default)]
    pub items: Vec<FontItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FontItem {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@itemID", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,

    #[serde(rename = "@font", skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,

    #[serde(rename = "@size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    #[serde(
        rename = "@sourceLastImported",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_last_imported: Option<u32>,

    #[serde(
        rename = "@embeddedCharacters",
        skip_serializing_if = "Option::is_none"
    )]
    pub embedded_characters: Option<String>,

    #[serde(rename = "@embedRanges", skip_serializing_if = "Option::is_none")]
    pub embed_ranges: Option<String>,
}
