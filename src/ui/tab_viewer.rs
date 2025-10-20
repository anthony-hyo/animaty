use crate::app::{AnimatyState, Tool};
use crate::ui::panel::Panel;
use eframe::egui::{Id, Pos2, Rect, Scene, UiBuilder, Vec2};
use egui_dock::tab_viewer::OnCloseResponse;
use egui_ltreeview::TreeView;

pub struct TabViewer<'a> {
    pub state: &'a mut AnimatyState,

    pub scene_rect: &'a mut Rect,

    pub active_tool: &'a mut Tool,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Panel;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.label().into()
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
