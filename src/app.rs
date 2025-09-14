#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console in release on Windows

use eframe::egui;
use eframe::egui::{Pos2, Rect, Scene, Vec2};
use egui_dock::egui::UiBuilder;
use egui_dock::{tab_viewer::OnCloseResponse, DockArea, DockState, NodeIndex, Style};

#[derive(Clone)]
enum Panel {
    Canvas,
    Tools,
    Properties,
    Timeline,
}

pub struct RufflyApp {
    tree: DockState<Panel>,

    project_name: String,

    canvas_width: f32,
    canvas_height: f32,

    scene_rect: Rect,
}

impl RufflyApp {
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl Default for RufflyApp {
    fn default() -> Self {
        // Start with a central canvas, and then create splits
        let mut tree = DockState::new(vec![Panel::Canvas]);

        // Create a column on the left with Tools; center remains the canvas
        let [left, center] =
            tree.main_surface_mut()
                .split_left(NodeIndex::root(), 0.20, vec![Panel::Tools]);

        // Split the left column top/bottom, adding Properties below
        let [_, _] = tree
            .main_surface_mut()
            .split_below(left, 0.6, vec![Panel::Properties]);

        // Add the Timeline below the center (canvas)
        let [_, _] = tree
            .main_surface_mut()
            .split_below(center, 0.85, vec![Panel::Timeline]);

        Self {
            tree,
            project_name: "New Project".to_owned(),

            canvas_width: 800.0,
            canvas_height: 600.0,

            scene_rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0)),
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
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Panel;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Panel::Canvas => "Canvas".into(),
            Panel::Tools => "Tools".into(),
            Panel::Properties => "Properties".into(),
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
                                        Rect::from_min_size(Pos2::ZERO, Vec2::new(600.0, 400.0)),
                                        0.0,
                                        egui::Color32::LIGHT_BLUE,
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
                if ui.button("🖱 Selection").clicked() {
                    println!("Tool: Selection");
                }
                if ui.button("✏ Pencil").clicked() {
                    println!("Tool: Pencil");
                }
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
                    if ui.button("⏮️").clicked() {}
                    if ui.button("⏸️").clicked() {}
                    if ui.button("▶️").clicked() {}
                    if ui.button("⏭️").clicked() {}
                    ui.separator();
                    ui.label("Frame: 1 / 30");
                });
            }
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
                        println!("Save clicked!");
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
                    project_name: &mut self.project_name,

                    canvas_width: &mut self.canvas_width,
                    canvas_height: &mut self.canvas_height,

                    scene_rect: &mut self.scene_rect,
                },
            );
    }
}
