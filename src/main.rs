use eframe::egui::{Style, Visuals};
use eframe::CreationContext;

mod app;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Ruffly v0.1",
        native_options,
        Box::new(|creation_context: &CreationContext| {
            let style = Style {
                visuals: Visuals::light(),
                ..Style::default()
            };

            creation_context.egui_ctx.set_style(style);

            Ok(Box::new(app::RufflyApp::new(creation_context)))
        }),
    )
}
