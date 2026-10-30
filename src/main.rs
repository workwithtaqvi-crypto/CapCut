// Hide the console window on Windows release builds.
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod app;
mod home;
mod icons;
mod sidebar;
mod theme;
mod widgets;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("LiteCut")
            .with_inner_size([1343.0, 896.0])
            .with_min_inner_size([960.0, 640.0])
            .with_decorations(false)
            .with_resizable(true),
        renderer: eframe::Renderer::Glow,
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "LiteCut",
        options,
        Box::new(|cc| Ok(Box::new(app::LiteCutApp::new(cc)))),
    )
}
