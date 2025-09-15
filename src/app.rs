#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console in release on Windows

use eframe::egui;
use eframe::egui::{Pos2, Rect, Scene, Vec2};
use egui_dock::egui::UiBuilder;
use egui_dock::{tab_viewer::OnCloseResponse, DockArea, DockState, NodeIndex, Style};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RufflyState {
    project_name: String,

    canvas_width: f32,
    canvas_height: f32,
}

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

pub struct RufflyApp {
    tree: DockState<Panel>,

    state: RufflyState,

    scene_rect: Rect,

    active_tool: Tool
}

impl RufflyApp {
    pub(crate) fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl Default for RufflyApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec![Panel::Canvas]);

        let [old, _new] = tree.main_surface_mut()
            .split_left(NodeIndex::root(), 0.20, vec![Panel::Tools]);

        let [_old, _new] = tree.main_surface_mut()
            .split_below(old, 0.85, vec![Panel::Timeline]);

        let [_old, _new] = tree.main_surface_mut()
            .split_right(old, 0.6, vec![Panel::Properties, Panel::Library]);

        Self {
            tree,

            state: RufflyState {
                project_name: "New Project".to_owned(),

                canvas_width: 800.0,
                canvas_height: 600.0,
            },

            scene_rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0)),

            active_tool: Tool::Selection
        }
    }
}

/// A viewer that will receive mutable references only to the necessary app fields.
/// This avoids borrow conflicts with `&mut self.tree`.
struct TabViewer<'a> {
    project_name: &'a mut String,

    canvas_width: &'a mut f32,
    canvas_height: &'a mut f32,

    scene_rect: &'a mut Rect,

    active_tool: &'a mut Tool
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
                                        Rect::from_min_size(Pos2::ZERO, Vec2::new(*self.canvas_width, *self.canvas_height)),
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

                ui
                    .selectable_value(self.active_tool, Tool::Selection, "🖱")
                    .on_hover_text("Selection");

                ui
                    .selectable_value(self.active_tool, Tool::Pencil, "✏")
                    .on_hover_text("Pencil");
            }

            Panel::Properties => {
                ui.label("Properties");

                ui.horizontal(|ui| {
                    ui.label("Project:");
                    ui.text_edit_singleline(self.project_name);
                });

                ui.add(egui::Slider::new(self.canvas_width, 100.0..=1920.0).text("Width"));
                ui.add(egui::Slider::new(self.canvas_height, 100.0..=1080.0).text("Height"));
            }

            Panel::Timeline => {
                ui.label("Timeline");

                ui.horizontal(|ui| {
                    if ui.button("⏮").on_hover_text("Go to start").clicked() {
                        println!("Timeline: back")
                    }
                    if ui.button("⏸").on_hover_text("Pause").clicked() {
                        println!("Timeline: pause");
                    }

                    if ui.button("▶").on_hover_text("Play").clicked() {
                        println!("Timeline: play");
                    }

                    if ui.button("⏭").on_hover_text("Go to end").clicked() {
                        println!("Timeline: end");
                    }

                    ui.separator();
                    ui.label("Frame: 1 / 30");
                });
            }
            Panel::Library => {
                ui.label("Timeline");
            },
        }
    }

    fn on_close(&mut self, _tab: &mut Self::Tab) -> OnCloseResponse {
        // Behavior when a tab is closed (can ask the user, save, etc.)
        OnCloseResponse::Close
    }
}

impl eframe::App for RufflyApp {
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
                    if ui.button("Save").clicked() {
                        if let Some(path) = rfd::FileDialog::new().add_filter("Ruffly Project", &["ruffly"]).save_file() {
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
                        println!("Ruffly v0.1 - Made with Rust!");
                    }
                });
            });
        });

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(
                ctx,
                &mut TabViewer {
                    project_name: &mut self.state.project_name,

                    canvas_width: &mut self.state.canvas_width,
                    canvas_height: &mut self.state.canvas_height,

                    scene_rect: &mut self.scene_rect,

                    active_tool: &mut self.active_tool
                },
            );
    }
}
