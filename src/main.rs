mod app;
mod sim;

use app::FluidApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 800.0])
            .with_title("Fluid Sandbox"),
        ..Default::default()
    };

    // Start egui app
    eframe::run_native(
        "Fluid Sandbox",
        options,
        Box::new(|_cc| Box::new(FluidApp::new())),
    )
}
