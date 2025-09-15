#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use eframe::{egui::{Style, Visuals, ViewportBuilder}, App, CreationContext};

mod app;
mod project;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "Ruffly v0.1",
        native_options,
         
         Box::new(|creation_context: &CreationContext| -> Result<Box<dyn App>, Box<dyn Error + Send + Sync>> {
            let style: Style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };

            creation_context.egui_ctx.set_style(style);

            Ok(Box::new(app::RufflyApp::new(creation_context)))
        }),
    )
}
