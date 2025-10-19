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
    #[serde(rename = "@index")]
    pub index: Option<u32>,

    #[serde(rename = "@duration")]
    pub duration: Option<u32>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@labelType")]
    pub label_type: Option<String>,

    #[serde(rename = "@keyMode")]
    pub key_mode: Option<u32>,

    #[serde(rename = "@tweenType")]
    pub tween_type: Option<String>,

    #[serde(rename = "@soundName")]
    pub sound_name: Option<String>,

    #[serde(rename = "@soundSync")]
    pub sound_sync: Option<String>,

    #[serde(rename = "SoundEnvelope")]
    pub sound_envelope: Option<SoundEnvelope>,

    #[serde(rename = "@motionTweenScale")]
    pub motion_tween_scale: Option<bool>,

    #[serde(rename = "MorphShape", default)]
    pub morph_shape: Vec<MorphShape>,

    #[serde(rename = "elements", default)]
    pub elements: Option<Elements>,
}
