#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

mod noise;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct App {
    noise_window: noise::Noise2D,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
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

                        ui.toggle_value(&mut self.noise_window.is_open, "2D Noise");

                        ui.separator();

                        if ui.button("Organize windows").clicked() {
                            ui.ctx().memory_mut(|mem| mem.reset_areas());
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
        self.noise_window.window(ctx);
    }
}
