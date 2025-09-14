use eframe::egui;
use eframe::egui::{Rect, Response};

pub struct RufflyApp {
    pub project_name: String,

    pub canvas_width: f32,
    pub canvas_height: f32,
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
        Self {
            project_name: "New Project".to_string(),
            canvas_width: 800.0,
            canvas_height: 600.0,
        }
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

                    ui.separator();

                    if ui.button("Export SWF").clicked() {
                        println!("Export SWF clicked!");
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        println!("Ruffly v0.1 - Made with Rust!");
                    }
                });
            });
        });

        // Left side panel (tools)
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();

            if ui.button("🖱️ Selection").clicked() {
                println!("Tool: Selection");
            }
            if ui.button("✏️ Pencil").clicked() {
                println!("Tool: Pencil");
            }
            if ui.button("⬛ Rectangle").clicked() {
                println!("Tool: Rectangle");
            }
            if ui.button("⭕ Circle").clicked() {
                println!("Tool: Circle");
            }
        });

        // Left side panel (tools)
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.separator();
            ui.heading("Properties");
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.project_name);
            });
            ui.add(egui::Slider::new(&mut self.canvas_width, 100.0..=1920.0).text("Width"));
            ui.add(egui::Slider::new(&mut self.canvas_height, 100.0..=1080.0).text("Height"));
        });

        // Bottom panel (timeline)
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.heading("Timeline");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("⏮️").clicked() {
                    println!("Go to start");
                }
                if ui.button("⏸️").clicked() {
                    println!("Pause");
                }
                if ui.button("▶️").clicked() {
                    println!("Play");
                }
                if ui.button("⏭️").clicked() {
                    println!("Go to end");
                }

                ui.separator();
                ui.label("Frame: 1 / 30");
            });
        });

        // Central panel (canvas)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drawing Canvas");

            // Drawing area
            let canvas_rect: Rect = ui.available_rect_before_wrap();
            let response: Response = ui.allocate_rect(canvas_rect, egui::Sense::click_and_drag());

            if ui.is_rect_visible(canvas_rect) {
                let painter = ui.painter();

                // White background
                painter.rect_filled(canvas_rect, 0.0, egui::Color32::WHITE);

                // Borders
                painter.rect_stroke(
                    canvas_rect,
                    0.0,
                    egui::Stroke::new(2.0, egui::Color32::BLACK),
                    egui::StrokeKind::Inside,
                );

                // Example text
                painter.text(
                    canvas_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Drawing Area",
                    egui::FontId::default(),
                    egui::Color32::GRAY,
                );
            }

            // Detect clicks on the canvas
            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    println!("Clicked on canvas at position: {:?}", pos);
                }
            }
        });
    }
}
