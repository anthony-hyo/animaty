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
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@itemID", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,

    #[serde(
        rename = "@sourceExternalFilepath",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_external_filepath: Option<String>,

    #[serde(
        rename = "@sourceLastImported",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_last_imported: Option<u32>,

    #[serde(rename = "@externalFileCRC32", skip_serializing_if = "Option::is_none")]
    pub external_file_crc32: Option<u32>,

    #[serde(rename = "@externalFileSize", skip_serializing_if = "Option::is_none")]
    pub external_file_size: Option<u32>,

    #[serde(rename = "@allowSmoothing", skip_serializing_if = "Option::is_none")]
    pub allow_smoothing: Option<bool>,

    #[serde(
        rename = "@useImportedJPEGData",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_imported_jpeg_data: Option<bool>,

    #[serde(rename = "@compressionType", skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,

    #[serde(
        rename = "@originalCompressionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_compression_type: Option<String>,

    #[serde(rename = "@quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<u32>,

    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,

    #[serde(rename = "@bitmapDataHRef", skip_serializing_if = "Option::is_none")]
    pub bitmap_data_href: Option<String>,

    #[serde(rename = "@frameRight", skip_serializing_if = "Option::is_none")]
    pub frame_right: Option<u32>,

    #[serde(rename = "@frameBottom", skip_serializing_if = "Option::is_none")]
    pub frame_bottom: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CompiledClipItem {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@itemID", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,

    #[serde(
        rename = "@linkageExportForAS",
        skip_serializing_if = "Option::is_none"
    )]
    pub linkage_export_for_as: Option<bool>,

    #[serde(rename = "@linkageClassName", skip_serializing_if = "Option::is_none")]
    pub linkage_class_name: Option<String>,

    #[serde(
        rename = "@sourceLastImported",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_last_imported: Option<u32>,

    #[serde(
        rename = "@displayAsComponent",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_as_component: Option<bool>,

    #[serde(rename = "@customIconID", skip_serializing_if = "Option::is_none")]
    pub custom_icon_id: Option<String>,

    #[serde(rename = "@actionscriptClass", skip_serializing_if = "Option::is_none")]
    pub actionscript_class: Option<String>,

    #[serde(rename = "@swfScmHRef", skip_serializing_if = "Option::is_none")]
    pub swf_scm_href: Option<String>,

    #[serde(
        rename = "@swfScmSourceFilename",
        skip_serializing_if = "Option::is_none"
    )]
    pub swf_scm_source_filename: Option<String>,

    #[serde(
        rename = "@persistLivePreview11",
        skip_serializing_if = "Option::is_none"
    )]
    pub persist_live_preview_11: Option<bool>,

    #[serde(
        rename = "@livePreview11ScmHRef",
        skip_serializing_if = "Option::is_none"
    )]
    pub live_preview_11_scm_href: Option<String>,

    #[serde(
        rename = "@livePreview11ScmSourceFilename",
        skip_serializing_if = "Option::is_none"
    )]
    pub live_preview_11_scm_source_filename: Option<String>,

    #[serde(rename = "@editFrameIndex", skip_serializing_if = "Option::is_none")]
    pub edit_frame_index: Option<u32>,

    #[serde(
        rename = "@requiredMinimumPlayerVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub required_minimum_player_version: Option<u32>,

    #[serde(
        rename = "@requiredMinimumASVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub required_minimum_as_version: Option<u32>,

    #[serde(
        rename = "@parametersAreLocked",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters_are_locked: Option<bool>,

    #[serde(rename = "@swcPath", skip_serializing_if = "Option::is_none")]
    pub swc_path: Option<String>,

    #[serde(
        rename = "@rootSymbolLinkageID",
        skip_serializing_if = "Option::is_none"
    )]
    pub root_symbol_linkage_id: Option<String>,

    #[serde(rename = "@playerVersion", skip_serializing_if = "Option::is_none")]
    pub player_version: Option<u32>,

    #[serde(
        rename = "@actionscriptVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub actionscript_version: Option<u32>,

    #[serde(rename = "@hashKey", skip_serializing_if = "Option::is_none")]
    pub hash_key: Option<String>,

    #[serde(rename = "classProperties", skip_serializing_if = "Option::is_none")]
    pub class_properties: Option<String>,

    #[serde(rename = "SwcItem", default)]
    pub swc_items: Vec<SwcItem>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SwcItem {
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,

    #[serde(rename = "@isTopLevel", skip_serializing_if = "Option::is_none")]
    pub is_top_level: Option<bool>,
}
