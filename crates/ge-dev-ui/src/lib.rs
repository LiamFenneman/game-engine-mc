#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::default_trait_access)]
//
#![feature(lint_reasons)]

#[macro_use]
extern crate tracing;

mod noise;
use noise::{Noise1D, Noise2D};

pub(crate) trait Window {
    fn is_open(&mut self) -> &mut bool;
    fn window(&mut self, ctx: &egui::Context);
    fn title(&self) -> &str;
}

pub(crate) struct App {
    windows: Vec<Box<dyn Window>>,
}

impl Default for App {
    fn default() -> Self {
        return Self {
            windows: vec![
                Box::<Noise1D>::default(),
                Box::<Noise2D>::default(),
            ],
        };
    }
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

                        for window in &mut self.windows {
                            let title = window.title().to_owned();
                            ui.toggle_value(window.is_open(), title);
                        }

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

        for window in &mut self.windows {
            window.window(ctx);
        }
    }
}

pub fn run_app() {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    _ = eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| return Box::<App>::default()),
    );
}
