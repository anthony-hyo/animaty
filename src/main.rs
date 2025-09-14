use eframe::HardwareAcceleration;

mod app;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Ruffly v0.1",
        eframe::NativeOptions {
            viewport: Default::default(),
            vsync: false,
            multisampling: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            hardware_acceleration: HardwareAcceleration::Required,
            renderer: Default::default(),
            run_and_return: false,
            event_loop_builder: None,
            window_builder: None,
            shader_version: None,
            centered: false,
            persist_window: false,
            persistence_path: None,
            dithering: false,
        },
        Box::new(|cc| Ok(Box::new(app::RufflyApp::new(cc)))),
    )
}
