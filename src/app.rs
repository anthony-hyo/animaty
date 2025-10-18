use eframe::egui::{Id, Pos2, Rect, Scene, UiBuilder, Vec2};
use egui_dock::{DockArea, DockState, NodeIndex, Style, tab_viewer::OnCloseResponse};
use egui_ltreeview::TreeView;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

use crate::project::{
    dom_document::DOMDocument,
    timeline::{Keyframe, Layer, Timeline},
};

use std::{fs, io::Read, str};

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    Selection,
    Pencil,
}

#[derive(Clone)]
enum Panel {
    Canvas,
    Tools,
    Properties,
    Library,
    Timeline,
}

struct TabViewer<'a> {
    state: &'a mut AnimatyState,

    scene_rect: &'a mut Rect,

    active_tool: &'a mut Tool,
}

#[derive(Serialize, Deserialize)]
struct AnimatyState {
    project_name: String,

    canvas_width: f32,
    canvas_height: f32,

    timeline: Timeline,
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

                                    // TODO: Temporary
                                    fs::write("output.txt", &xml_content)
                                        .expect("Unable to write file");

                                    println!("DOMDocument raw");

                                    let dom_document: DOMDocument = match from_str(&xml_content) {
                                        Ok(doc) => {
                                            doc
                                        },
                                        Err(e) => {
                                            eprintln!("Error deserializing DOMDocument.xml: {}", e);
                                            return;
                                        }
                                    };

                                    println!("DOMDocument parsed: {:#?}", dom_document);
                                }
                                Err(_) => {
                                    eprintln!("Error: DOMDocument.xml not found inside the .fla file.");
                                }
                            }
                        }
                    }

                    if ui.button("Save").clicked() {
                        if let Some(path) = rfd::FileDialog::new().add_filter("Animaty Project", &["animty"]).save_file() {
                            let json_data = serde_json::to_string_pretty(&self.state)
                            .expect("Error serializing project state");

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

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Panel;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Panel::Canvas => "Canvas".into(),
            Panel::Tools => "Tools".into(),
            Panel::Properties => "Properties".into(),
            Panel::Library => "Library".into(),
            Panel::Timeline => "Timeline".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Panel::Canvas => {
                ui.vertical_centered(|ui| ui.heading("Canvas"));

                egui::Frame::group(ui.style())
                    .inner_margin(0.0)
                    .show(ui, |ui| {
                        let scene = Scene::new()
                            .max_inner_size([2000.0, 2000.0])
                            .zoom_range(0.1..=4.0);

                        let mut reset_view = false;
                        let mut inner_rect = Rect::NAN;

                        let response = scene
                            .show(ui, self.scene_rect, |ui| {
                                reset_view = ui.button("Reset view").clicked();

                                let rect = Rect::from_min_size(
                                    Pos2::new(0.0, 0.0),
                                    Vec2::new(600.0, 400.0),
                                );

                                ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                                    ui.painter().rect_filled(
                                        Rect::from_min_size(
                                            Pos2::ZERO,
                                            Vec2::new(
                                                *&mut self.state.canvas_width,
                                                *&mut self.state.canvas_height,
                                            ),
                                        ),
                                        0.0,
                                        egui::Color32::DARK_GRAY,
                                    );
                                });

                                inner_rect = ui.min_rect();
                            })
                            .response;

                        if reset_view || response.double_clicked() {
                            *self.scene_rect = inner_rect;
                        }
                    });
            }

            Panel::Tools => {
                ui.label("Tools");

                ui.selectable_value(self.active_tool, Tool::Selection, "🖱")
                    .on_hover_text("Selection");

                ui.selectable_value(self.active_tool, Tool::Pencil, "✏")
                    .on_hover_text("Pencil");
            }

            Panel::Properties => {
                ui.label("Properties");

                ui.horizontal(|ui| {
                    ui.label("Project:");
                    ui.text_edit_singleline(&mut self.state.project_name);
                });

                ui.add(
                    egui::Slider::new(&mut self.state.canvas_width, 100.0..=1920.0).text("Width"),
                );
                ui.add(
                    egui::Slider::new(&mut self.state.canvas_height, 100.0..=1080.0).text("Height"),
                );
            }

            Panel::Timeline => {
                ui.label("Timeline");

                ui.horizontal(|ui| {
                    if ui.button("⏮").on_hover_text("Go to start").clicked() {
                        self.state.timeline.current_frame = 1;
                    }
                    if ui.button("⏸").on_hover_text("Pause").clicked() {
                        println!("Timeline: pause");
                    }

                    if ui.button("▶").on_hover_text("Play").clicked() {
                        println!("Timeline: play");
                    }

                    if ui.button("⏭").on_hover_text("Go to end").clicked() {
                        let timeline = &mut self.state.timeline;

                        timeline.current_frame += 1;

                        if timeline.current_frame > timeline.total_frames {
                            timeline.current_frame = timeline.total_frames;
                        }
                    }

                    ui.separator();

                    let timeline_state = &self.state.timeline;

                    ui.label(format!(
                        "Frame: {} / {}",
                        timeline_state.current_frame, timeline_state.total_frames
                    ));
                });
            }

            Panel::Library => {
                ui.label("Timeline");

                TreeView::new(Id::new("tree view2")).show(ui, |builder| {
                    builder.dir(0, "Root");
                    builder.leaf(1, "Ava");
                    builder.leaf(2, "Benjamin");
                    builder.leaf(3, "Charlotte");
                    builder.close_dir();

                    builder.dir(4, "Root2");
                    builder.leaf(5, "Ava");
                    builder.leaf(6, "Benjamin");
                    builder.leaf(7, "Charlotte");

                    builder.close_dir();
                });
            }
        }
    }

    fn on_close(&mut self, _tab: &mut Self::Tab) -> OnCloseResponse {
        // Behavior when a tab is closed (can ask the user, save, etc.)
        OnCloseResponse::Close
    }
}
