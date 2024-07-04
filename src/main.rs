#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod base2_16;
mod base16_2;
mod base32_f32;
mod basef32_32;
mod data;

use base2_16::*;
use base16_2::*;
use base32_f32::*;
use basef32_32::*;
use data::*;
use eframe::egui;
use egui::*;
fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([850.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native("进制转换", options, Box::new(|cc| Box::new(App::new(cc))))
}

struct App {
    base2_16: Data,
    base16_2: Data,
    base32_f32: Data,
    basef32_32: Data,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            base2_16: Data::new(),
            base16_2: Data::new(),
            base32_f32: Data::new(),
            basef32_32: Data::new(),
        }
    }
    fn base2_16(&mut self, ui: &mut Ui) {
        base2_16(&mut self.base2_16, ui);
    }
    fn base16_2(&mut self, ui: &mut Ui) {
        base16_2(&mut self.base16_2, ui);
    }
    fn base32_f32(&mut self, ui: &mut Ui) {
        base32_f32(&mut self.base32_f32, ui);
    }
    fn basef32_32(&mut self, ui: &mut Ui) {
        basef32_32(&mut self.basef32_32, ui);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.base2_16(ui);
            self.base16_2(ui);
            self.basef32_32(ui);
            self.base32_f32(ui);
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Song".to_owned(),
        egui::FontData::from_static(include_bytes!("./STSong.ttf")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Song".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("Song".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
