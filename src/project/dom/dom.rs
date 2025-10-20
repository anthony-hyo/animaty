use crate::project::dom::{
    dom_extended_swatch_lists::DOMExtendedSwatchLists, dom_folder::DOMFolders, dom_font::DOMFonts,
    dom_media::DOMMedia, dom_persistent_data::DOMPersistentData,
    dom_printer_setting::DOMPrinterSettings, dom_publish_history::DOMPublishHistory,
    dom_swatch_lists::DOMSwatchLists, dom_symbol::DOMSymbols,
    dom_timeline::dom_timeline::DOMTimelines,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOM {
    #[serde(rename = "@backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,

    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(rename = "@currentTimeline", skip_serializing_if = "Option::is_none")]
    pub current_timeline: Option<u32>,
    #[serde(rename = "@xflVersion", skip_serializing_if = "Option::is_none")]
    pub xfl_version: Option<f32>,
    #[serde(rename = "@creatorInfo", skip_serializing_if = "Option::is_none")]
    pub creator_info: Option<String>,
    #[serde(rename = "@platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "@versionInfo", skip_serializing_if = "Option::is_none")]
    pub version_info: Option<String>,
    #[serde(rename = "@majorVersion", skip_serializing_if = "Option::is_none")]
    pub major_version: Option<u32>,
    #[serde(rename = "@buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<u32>,
    #[serde(rename = "@viewAngle3D", skip_serializing_if = "Option::is_none")]
    pub view_angle_3d: Option<f32>,
    #[serde(
        rename = "@nextSceneIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_scene_identifier: Option<u32>,
    #[serde(
        rename = "@playOptionsPlayLoop",
        skip_serializing_if = "Option::is_none"
    )]
    pub play_options_play_loop: Option<bool>,
    #[serde(
        rename = "@playOptionsPlayPages",
        skip_serializing_if = "Option::is_none"
    )]
    pub play_options_play_pages: Option<bool>,
    #[serde(
        rename = "@playOptionsPlayFrameActions",
        skip_serializing_if = "Option::is_none"
    )]
    pub play_options_play_frame_actions: Option<bool>,
    #[serde(rename = "@filetypeGUID", skip_serializing_if = "Option::is_none")]
    pub filetype_guid: Option<String>,

    #[serde(rename = "folders", default)]
    pub folders: Option<DOMFolders>,

    #[serde(rename = "fonts", default)]
    pub fonts: Option<DOMFonts>,

    #[serde(rename = "media", default)]
    pub media: Option<DOMMedia>,

    #[serde(rename = "symbols", default)]
    pub symbols: Option<DOMSymbols>,

    #[serde(rename = "timelines", default)]
    pub timelines: Option<DOMTimelines>,

    #[serde(rename = "swatchLists", default)]
    pub swatch_lists: Option<DOMSwatchLists>,

    #[serde(rename = "extendedSwatchLists", default)]
    pub extended_swatch_lists: Option<DOMExtendedSwatchLists>,

    #[serde(rename = "persistentData", default)]
    pub persistent_data: Option<DOMPersistentData>,

    #[serde(rename = "PrinterSettings", default)]
    pub printer_settings: Option<DOMPrinterSettings>,

    #[serde(rename = "publishHistory", default)]
    pub publish_history: Option<DOMPublishHistory>,
}
