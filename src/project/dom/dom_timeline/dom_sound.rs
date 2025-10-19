use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SoundEnvelope {
    #[serde(rename = "SoundEnvelopePoint", default)]
    pub points: Vec<SoundEnvelopePoint>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SoundEnvelopePoint {
    #[serde(rename = "@level0")]
    pub level0: Option<u32>,

    #[serde(rename = "@level1")]
    pub level1: Option<u32>,
}
