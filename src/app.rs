use eframe::egui::{Id, Pos2, Rect, Scene, UiBuilder, Vec2};
use egui_dock::{DockArea, DockState, NodeIndex, Style, tab_viewer::OnCloseResponse};
use egui_ltreeview::TreeView;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

use crate::project::{
    DOMDocument,
    timeline::{Keyframe, Layer, Timeline},
};

use crate::ui::panel::Panel;
use crate::ui::tab_viewer::TabViewer;
use quick_xml::se::to_string;
use std::{fs, io::Read, str};

#[derive(Clone, Copy, PartialEq)]
pub enum Tool {
    Selection,
    Pencil,
}

#[derive(Serialize, Deserialize)]
pub struct AnimatyState {
    pub project_name: String,

    pub canvas_width: f32,
    pub canvas_height: f32,

    pub timeline: Timeline,
}

pub struct AnimatyApp {
    tree: DockState<Panel>,

    state: AnimatyState,

    scene_rect: Rect,

    active_tool: Tool,
}

impl AnimatyApp {
    pub(crate) fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl Default for AnimatyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec![Panel::Canvas]);

        let [old, _new] =
            tree.main_surface_mut()
                .split_left(NodeIndex::root(), 0.20, vec![Panel::Tools]);

        let [_old, _new] = tree
            .main_surface_mut()
            .split_below(old, 0.85, vec![Panel::Timeline]);

        let [_old, _new] =
            tree.main_surface_mut()
                .split_right(old, 0.6, vec![Panel::Properties, Panel::Library]);

        Self {
            tree,

            state: AnimatyState {
                project_name: "New Project".to_owned(),

                canvas_width: 800.0,
                canvas_height: 600.0,

                timeline: Timeline {
                    layers: vec![Layer {
                        name: "Layer 1".to_owned(),
                        is_visible: true,
                        keyframes: vec![Keyframe {
                            frame_number: 1,
                            drawing: Vec::new(),
                        }],
                    }],
                    current_frame: 1,
                    total_frames: 30,
                    fps: 24,
                },
            },

            scene_rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0)),

            active_tool: Tool::Selection,
        }
    }
}

impl eframe::App for AnimatyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::MenuBar::new().ui(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("New Project").clicked() {
						println!("New project clicked!");
					}

					if ui.button("Open").clicked() {
						println!("Open clicked!");
					}

					if ui.button("Import .fla...").clicked() {
						if let Some(path) = rfd::FileDialog::new().add_filter("Flash Project", &["fla"]).pick_file() {
							println!("Attempting to import FLA: {:?}", path);

							let file = match std::fs::File::open(&path) {
								Ok(f) => f,
								Err(e) => {
									eprintln!("Error opening file: {}", e);
									return;
								}
							};

							let mut archive = match zip::ZipArchive::new(file) {
								Ok(a) => a,
								Err(e) => {
									eprintln!("Error reading as ZIP archive (is this a modern .fla file?): {}", e);
									return;
								}
							};

							match archive.by_name("DOMDocument.xml") {
								Ok(mut xml_file) => {
									let mut xml_content: String = String::new();

									if let Err(e) = xml_file.read_to_string(&mut xml_content) {
										eprintln!("Error reading DOMDocument.xml content: {}", e);
										return;
									}

									// TODO: Remove | Temporary
									fs::write("output_original.xml", &xml_content).expect("Unable to write file");

									println!("DOMDocument raw");

									let dom_document: DOMDocument = match from_str(&xml_content) {
										Ok(doc) => {
											doc
										}
										Err(e) => {
											eprintln!("Error deserializing DOMDocument.xml: {}", e);
											return;
										}
									};

									// TODO: Remove | Temporary
									fs::write("output.json", serde_json::to_string_pretty(&dom_document)
										.expect("Error serializing to JSON")
									).expect("Unable to write file");

									// TODO: Remove | Temporary
									fs::write("output.xml", to_string(&dom_document)
										.expect("Error serializing to XML")
									).expect("Unable to write file");

									println!("DOMDocument parsed");
								}
								Err(_) => {
									eprintln!("Error: DOMDocument.xml not found inside the .fla file.");
								}
							}
						}
					}

					if ui.button("Save").clicked() {
						if let Some(path) = rfd::FileDialog::new().add_filter("Animaty Project", &["animty"]).save_file() {
							let json_data = serde_json::to_string_pretty(&self.state).expect("Error serializing project state");

							if let Err(e) = std::fs::write(&path, json_data) {
								println!("Error saving the file: {}", e);
							} else {
								println!("Project saved successfully: {:?}", path);
							}
						}
					}
				});
				ui.menu_button("Help", |ui| {
					if ui.button("About").clicked() {
						println!("Animaty v0.1 - Made with Rust!");
					}
				});
			});
		});

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(
                ctx,
                &mut TabViewer {
                    state: &mut self.state,

                    scene_rect: &mut self.scene_rect,

                    active_tool: &mut self.active_tool,
                },
            );
    }
}
