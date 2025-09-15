use egui::Pos2;
use serde::{Deserialize, Serialize};

type Drawing = Vec<Stroke>;

#[derive(Serialize, Deserialize)]
pub struct Timeline {
    pub layers: Vec<Layer>,
    pub current_frame: u32,
    pub total_frames: u32,
    pub fps: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub is_visible: bool,
    //TODO: Is Locked Layer
    //TODO: Is Guide Layer
    pub keyframes: Vec<Keyframe>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub frame_number: u32,
    pub drawing: Drawing,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub points: Vec<Pos2>,
    //TODO: color
    //TODO: thickness
}