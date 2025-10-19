use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMFonts {
    #[serde(rename = "DOMFontItem", default)]
    pub items: Vec<FontItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FontItem {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@itemID")]
    pub item_id: Option<String>,

    #[serde(rename = "@font")]
    pub font: Option<String>,

    #[serde(rename = "@size")]
    pub size: Option<u32>,

    #[serde(rename = "@id")]
    pub id: Option<u32>,

    #[serde(rename = "@sourceLastImported")]
    pub source_last_imported: Option<u32>,

    #[serde(rename = "@embeddedCharacters")]
    pub embedded_characters: Option<String>,

    #[serde(rename = "@embedRanges")]
    pub embed_ranges: Option<String>,
}
