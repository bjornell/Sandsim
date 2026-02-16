use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};
use crate::sim::Simulation;

pub struct FluidApp {
    sim: Simulation,
    gravity_angle: f32,
    gravity_strength: f32,
}

impl FluidApp {
    pub fn new() -> Self {
        let width = 30;
        let height = 30;
        Self {
            sim: Simulation::new(width, height),
            gravity_angle: 90.0,   // pointing down
            gravity_strength: 1.0,
        }
    }

    fn update_gravity(&mut self) {
        let rad = self.gravity_angle.to_radians();
        let gravity = Vec2::new(rad.cos(), rad.sin()) * self.gravity_strength;
        self.sim.set_gravity(gravity);
    }

    fn draw_grid(&self, painter: &Painter, top_left: Pos2, cell_size: f32) {
        for y in 0..self.sim.height {
            for x in 0..self.sim.width {
                let idx = y * self.sim.width + x;
                let d = self.sim.density[idx].clamp(0.0, 1.0);

                let rect = Rect::from_min_size(
                    top_left + Vec2::new(x as f32 * cell_size, y as f32 * cell_size),
                    Vec2::splat(cell_size),
                );

                // Fill color: red for sand, gray for empty
                let color = if d > 0.0 {
                    Color32::from_rgb((d * 255.0) as u8, 0, 0)
                } else {
                    Color32::from_rgb(30, 30, 30)
                };

                painter.rect_filled(rect, 0.0, color);
                painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::GRAY));
            }
        }
    }
}

impl eframe::App for FluidApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Controls");

            ui.add(
                egui::Slider::new(&mut self.gravity_angle, 0.0..=360.0)
                    .text("Gravity angle"),
            );
            ui.add(
                egui::Slider::new(&mut self.gravity_strength, 0.0..=5.0)
                    .text("Gravity strength"),
            );

            if ui.button("Reset").clicked() {
                self.sim.reset();
            }
        });

        self.update_gravity();
        self.sim.step();

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter_at(ui.max_rect());
            let panel_rect = ui.available_rect_before_wrap();

            let cell_size = 20.0;
            let grid_width = self.sim.width as f32 * cell_size;
            let grid_height = self.sim.height as f32 * cell_size;

            // Center the grid in the panel
            let top_left = panel_rect.min + egui::vec2(
                (panel_rect.width() - grid_width) / 2.0,
                (panel_rect.height() - grid_height) / 2.0,
            );

            self.draw_grid(&painter, top_left, cell_size);
        });

        ctx.request_repaint();
    }
}
