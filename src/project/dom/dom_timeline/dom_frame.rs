use crate::project::dom::dom_timeline::dom_element::Elements;
use crate::project::dom::dom_timeline::dom_morph::MorphShape;
use crate::project::dom::dom_timeline::dom_sound::SoundEnvelope;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Frames {
    #[serde(rename = "DOMFrame", default)]
    pub frames: Vec<Frame>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Frame {
    #[serde(rename = "@index", skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,

    #[serde(rename = "@duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,

    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "@labelType", skip_serializing_if = "Option::is_none")]
    pub label_type: Option<String>,

    #[serde(rename = "@keyMode", skip_serializing_if = "Option::is_none")]
    pub key_mode: Option<u32>,

    #[serde(rename = "@tweenType", skip_serializing_if = "Option::is_none")]
    pub tween_type: Option<String>,

    #[serde(rename = "@soundName", skip_serializing_if = "Option::is_none")]
    pub sound_name: Option<String>,

    #[serde(rename = "@soundSync", skip_serializing_if = "Option::is_none")]
    pub sound_sync: Option<String>,

    #[serde(rename = "SoundEnvelope", skip_serializing_if = "Option::is_none")]
    pub sound_envelope: Option<SoundEnvelope>,

    #[serde(rename = "@motionTweenScale", skip_serializing_if = "Option::is_none")]
    pub motion_tween_scale: Option<bool>,

    #[serde(rename = "MorphShape", default)]
    pub morph_shape: Vec<MorphShape>,

    #[serde(rename = "elements", default)]
    pub elements: Option<Elements>,
}
