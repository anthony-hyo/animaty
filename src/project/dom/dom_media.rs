use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DOMMedia {
    #[serde(rename = "$value")]
    pub items: Vec<MediaItems>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde()]
pub enum MediaItems {
    DOMBitmapItem(BitmapItem),
    DOMCompiledClipItem(CompiledClipItem),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BitmapItem {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@itemID")]
    pub item_id: Option<String>,

    #[serde(rename = "@sourceExternalFilepath")]
    pub source_external_filepath: Option<String>,

    #[serde(rename = "@sourceLastImported")]
    pub source_last_imported: Option<u32>,

    #[serde(rename = "@externalFileCRC32")]
    pub external_file_crc32: Option<u32>,

    #[serde(rename = "@externalFileSize")]
    pub external_file_size: Option<u32>,

    #[serde(rename = "@allowSmoothing")]
    pub allow_smoothing: Option<bool>,

    #[serde(rename = "@useImportedJPEGData")]
    pub use_imported_jpeg_data: Option<bool>,

    #[serde(rename = "@compressionType")]
    pub compression_type: Option<String>,

    #[serde(rename = "@originalCompressionType")]
    pub original_compression_type: Option<String>,

    #[serde(rename = "@quality")]
    pub quality: Option<u32>,

    #[serde(rename = "@href")]
    pub href: Option<String>,

    #[serde(rename = "@bitmapDataHRef")]
    pub bitmap_data_href: Option<String>,

    #[serde(rename = "@frameRight")]
    pub frame_right: Option<u32>,

    #[serde(rename = "@frameBottom")]
    pub frame_bottom: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CompiledClipItem {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@itemID")]
    pub item_id: Option<String>,

    #[serde(rename = "@linkageExportForAS")]
    pub linkage_export_for_as: Option<bool>,

    #[serde(rename = "@linkageClassName")]
    pub linkage_class_name: Option<String>,

    #[serde(rename = "@sourceLastImported")]
    pub source_last_imported: Option<u32>,

    #[serde(rename = "@displayAsComponent")]
    pub display_as_component: Option<bool>,

    #[serde(rename = "@customIconID")]
    pub custom_icon_id: Option<String>,

    #[serde(rename = "@actionscriptClass")]
    pub actionscript_class: Option<String>,

    #[serde(rename = "@swfScmHRef")]
    pub swf_scm_href: Option<String>,

    #[serde(rename = "@swfScmSourceFilename")]
    pub swf_scm_source_filename: Option<String>,

    #[serde(rename = "@persistLivePreview11")]
    pub persist_live_preview_11: Option<bool>,

    #[serde(rename = "@livePreview11ScmHRef")]
    pub live_preview_11_scm_href: Option<String>,

    #[serde(rename = "@livePreview11ScmSourceFilename")]
    pub live_preview_11_scm_source_filename: Option<String>,

    #[serde(rename = "@editFrameIndex")]
    pub edit_frame_index: Option<u32>,

    #[serde(rename = "@requiredMinimumPlayerVersion")]
    pub required_minimum_player_version: Option<u32>,

    #[serde(rename = "@requiredMinimumASVersion")]
    pub required_minimum_as_version: Option<u32>,

    #[serde(rename = "@parametersAreLocked")]
    pub parameters_are_locked: Option<bool>,

    #[serde(rename = "@swcPath")]
    pub swc_path: Option<String>,

    #[serde(rename = "@rootSymbolLinkageID")]
    pub root_symbol_linkage_id: Option<String>,

    #[serde(rename = "@playerVersion")]
    pub player_version: Option<u32>,

    #[serde(rename = "@actionscriptVersion")]
    pub actionscript_version: Option<u32>,

    #[serde(rename = "@hashKey")]
    pub hash_key: Option<String>,

    #[serde(rename = "classProperties")]
    pub class_properties: Option<String>,

    #[serde(rename = "SwcItem", default)]
    pub swc_items: Vec<SwcItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SwcItem {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@version")]
    pub version: Option<u32>,

    #[serde(rename = "@isTopLevel")]
    pub is_top_level: Option<bool>,
}
