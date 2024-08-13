#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui_plotter::{
    plotters::{
        chart::ChartBuilder,
        prelude::IntoDrawingArea,
        style::{full_palette::WHITE, Color},
    },
    EguiBackend,
};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("performance");
            egui::Frame::default().show(ui, |ui| {
                let r = EguiBackend::new(ui).into_drawing_area();
                let mut cb = ChartBuilder::on(&r);
                let mut cx = cb.build_cartesian_2d(0..590, 0..59).unwrap();
                cx.configure_mesh()
                    .light_line_style(WHITE.mix(0.005))
                    .bold_line_style(WHITE.mix(0.01))
                    .draw()
                    .unwrap();
            });
        });
    }
}
