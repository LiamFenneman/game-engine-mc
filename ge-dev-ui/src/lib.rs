#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]

mod noise;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct App {
    win_noise1d: noise::Noise1D,
    win_noise2d: noise::Noise2D,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });
        egui::SidePanel::right("egui_demo_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.label("Windows");

                        ui.toggle_value(&mut self.win_noise1d.is_open, "1D Noise");
                        ui.toggle_value(&mut self.win_noise2d.is_open, "2D Noise");

                        ui.separator();

                        if ui.button("Organize windows").clicked() {
                            ui.ctx().memory_mut(egui::Memory::reset_areas);
                        }
                    });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                return {
                    egui::warn_if_debug_build(ui);
                };
            });
        });

        // windows
        self.win_noise1d.window(ctx);
        self.win_noise2d.window(ctx);
    }
}
