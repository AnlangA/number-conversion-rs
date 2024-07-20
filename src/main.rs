#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod base2;
mod base10;
mod base16;
mod base32_f32;
mod basef32_32;
mod data;

use base2::*;
use base10::*;
use base16::*;
use base32_f32::*;
use basef32_32::*;
use data::*;
use eframe::egui;
use egui::*;
use egui_extras::*;
fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native("进制转换", options, Box::new(|cc| Box::new(App::new(cc))))
}

struct App {
    base2: Data,
    base10: Data,
    base16: Data,
    base32_f32: Data,
    basef32_32: Data,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        install_image_loaders(&cc.egui_ctx);
        Self {
            base2: Data::new(),
            base10: Data::new(),
            base16: Data::new(),
            base32_f32: Data::new(),
            basef32_32: Data::new(),
        }
    }
    fn base2(&mut self, ui: &mut Ui) {
        base2(&mut self.base2, ui);
    }
    fn base10(&mut self, ui: &mut Ui){
        base10(&mut self.base10, ui);
    }
    fn base16(&mut self, ui: &mut Ui) {
        base16(&mut self.base16, ui);
    }
    fn base32_f32(&mut self, ui: &mut Ui) {
        base32_f32(&mut self.base32_f32, ui);
    }
    fn basef32_32(&mut self, ui: &mut Ui) {
        basef32_32(&mut self.basef32_32, ui);
    }
    fn github_link(&self, ctx: &egui::Context){
        egui::TopBottomPanel::bottom("链接")
            .show(ctx, |ui|{
                ui.add(egui::Hyperlink::from_label_and_url("😄 源码仓库", "https://github.com/AnlangA/number-conversion-rs"));
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.base2(ui);
            self.base10(ui);
            self.base16(ui);
            self.basef32_32(ui);
            self.base32_f32(ui);
            ui.centered_and_justified(|ui| {
                ui.image(include_image!("./picture/rust_zh.png"));
            });
            self.github_link(ctx);
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Song".to_owned(),
        egui::FontData::from_static(include_bytes!("./STSong.ttf")),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Song".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("Song".to_owned());

    ctx.set_fonts(fonts);
}
