use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SoundEnvelope {
    #[serde(rename = "SoundEnvelopePoint", default)]
    pub points: Vec<SoundEnvelopePoint>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SoundEnvelopePoint {
    #[serde(rename = "@level0", skip_serializing_if = "Option::is_none")]
    pub level0: Option<u32>,

    #[serde(rename = "@level1", skip_serializing_if = "Option::is_none")]
    pub level1: Option<u32>,
}
