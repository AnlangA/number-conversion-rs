#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use number_conversion::app::ApplicationBuilder;

fn main() -> Result<(), eframe::Error> {
    ApplicationBuilder::new()
        .with_title("编码转换工具")
        .with_window_size(900.0, 700.0)
        .with_logging(cfg!(debug_assertions))
        .run()
}
