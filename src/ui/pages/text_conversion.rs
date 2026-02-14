//! Text conversion page.

use eframe::egui::{self, Color32, RichText, Ui, TextEdit};
use crate::frontend::FrontendState;

/// Render the text conversion page.
pub fn render(ui: &mut Ui, frontend: &mut FrontendState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("文本转换");
        ui.add_space(10.0);

        // ASCII to Hex
        ui.group(|ui| {
            ui.label(RichText::new("ASCII → 十六进制").color(Color32::BLUE));
            
            let response = ui.add(
                TextEdit::singleline(&mut frontend.text_conversion.ascii_to_hex.input)
                    .desired_width(300.0)
                    .hint_text("输入ASCII文本，如: Hello"),
            );
            
            if response.changed() {
                frontend.request_text_conversion(true);
            }
            
            if frontend.text_conversion.ascii_to_hex.pending_id.is_some() {
                ui.spinner();
            }
            
            let field = &frontend.text_conversion.ascii_to_hex;
            if let Some(err) = &field.error {
                ui.colored_label(Color32::RED, err);
            } else if !field.output.is_empty() {
                ui.label("结果:");
                ui.monospace(&field.output);
            }
        });
        ui.add_space(8.0);

        // Hex to ASCII
        ui.group(|ui| {
            ui.label(RichText::new("十六进制 → ASCII").color(Color32::BLUE));
            
            let response = ui.add(
                TextEdit::singleline(&mut frontend.text_conversion.hex_to_ascii.input)
                    .desired_width(300.0)
                    .hint_text("输入十六进制，如: 48 65 6C 6C 6F"),
            );
            
            if response.changed() {
                frontend.request_text_conversion(false);
            }
            
            if frontend.text_conversion.hex_to_ascii.pending_id.is_some() {
                ui.spinner();
            }
            
            let field = &frontend.text_conversion.hex_to_ascii;
            if let Some(err) = &field.error {
                ui.colored_label(Color32::RED, err);
            } else if !field.output.is_empty() {
                ui.label("结果:");
                ui.monospace(&field.output);
            }
        });
        ui.add_space(8.0);

        // Action buttons
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("清除所有").clicked() {
                frontend.text_conversion.ascii_to_hex.input.clear();
                frontend.text_conversion.ascii_to_hex.output.clear();
                frontend.text_conversion.ascii_to_hex.error = None;
                frontend.text_conversion.ascii_to_hex.pending_id = None;
                
                frontend.text_conversion.hex_to_ascii.input.clear();
                frontend.text_conversion.hex_to_ascii.output.clear();
                frontend.text_conversion.hex_to_ascii.error = None;
                frontend.text_conversion.hex_to_ascii.pending_id = None;
            }
            
            if ui.button("加载示例").clicked() {
                frontend.text_conversion.ascii_to_hex.input = "Hello World!".to_string();
                frontend.request_text_conversion(true);
                
                frontend.text_conversion.hex_to_ascii.input = "48 65 6C 6C 6F 20 57 6F 72 6C 64 21".to_string();
                frontend.request_text_conversion(false);
            }
        });

        // Usage instructions
        ui.separator();
        ui.collapsing("使用说明", |ui| {
            ui.label("• ASCII转十六进制：将文本字符转换为对应的十六进制编码");
            ui.label("• 十六进制转ASCII：将十六进制编码转换为对应的文本字符");
            ui.label("• 十六进制输入支持空格分隔，如：48 65 6C 6C 6F");
            ui.label("• 不可打印字符将显示为 [0xXX] 格式");
        });
    });
}
