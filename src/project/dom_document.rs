use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMDocument {
    #[serde(rename = "@backgroundColor")]
    pub background_color: String,
    #[serde(rename = "@width")]
    pub width: u32,
    #[serde(rename = "@height")]
    pub height: u32,
    #[serde(rename = "@currentTimeline")]
    pub current_timeline: u32,
    #[serde(rename = "@xflVersion")]
    pub xfl_version: f32,
    #[serde(rename = "@creatorInfo")]
    pub creator_info: String,
    #[serde(rename = "@platform")]
    pub platform: String,
    #[serde(rename = "@versionInfo")]
    pub version_info: String,
    #[serde(rename = "@majorVersion")]
    pub major_version: u32,
    #[serde(rename = "@buildNumber")]
    pub build_number: u32,
    #[serde(rename = "@viewAngle3D")]
    pub view_angle_3d: f32,
    #[serde(rename = "@nextSceneIdentifier")]
    pub next_scene_identifier: u32,
    #[serde(rename = "@playOptionsPlayLoop")]
    pub play_options_play_loop: bool,
    #[serde(rename = "@playOptionsPlayPages")]
    pub play_options_play_pages: bool,
    #[serde(rename = "@playOptionsPlayFrameActions")]
    pub play_options_play_frame_actions: bool,
    #[serde(rename = "@filetypeGUID")]
    pub filetype_guid: String,

    #[serde(rename = "@folders")]
    pub folders: Option<DOMDocumentFolders>,
    #[serde(rename = "@symbols")]
    pub symbols: Option<DOMDocumentSymbols>,
    #[serde(rename = "@timelines")]
    pub timelines: Option<DOMDocumentTimelines>,
    #[serde(rename = "@persistentData")]
    pub persistent_data: Option<DOMDocumentPersistentData>,
    #[serde(rename = "@PrinterSettings")]
    pub printer_settings: Option<DOMDocumentPrinterSettings>,
    #[serde(rename = "@publishHistory")]
    pub publish_history: Option<DOMDocumentPublishHistory>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMDocumentFolders {}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMDocumentSymbols {}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMDocumentTimelines {}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMDocumentPersistentData {}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMDocumentPrinterSettings {}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMDocumentPublishHistory {}
