//! Number conversion page.

use eframe::egui::{self, Color32, RichText, Ui, TextEdit};
use crate::frontend::FrontendState;

/// Render the number conversion page.
pub fn render(ui: &mut Ui, frontend: &mut FrontendState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("进制转换");
        ui.add_space(10.0);

        // Binary converter
        ui.group(|ui| {
            ui.label(RichText::new("二进制转换").color(Color32::BLUE));
            
            let response = ui.add(
                TextEdit::singleline(&mut frontend.number_conversion.binary_field.input)
                    .desired_width(300.0)
                    .hint_text("输入二进制数，如: 1010"),
            );
            
            if response.changed() {
                frontend.request_binary_conversion();
            }
            
            if frontend.number_conversion.binary_field.pending_id.is_some() {
                ui.spinner();
            }
            
            let field = &frontend.number_conversion.binary_field;
            if let Some(err) = &field.error {
                ui.colored_label(Color32::RED, err);
            } else if !field.binary.is_empty() || !field.decimal.is_empty() || !field.hexadecimal.is_empty() {
                ui.horizontal(|ui| {
                    if !field.binary.is_empty() { ui.label(format!("2进制: {}", field.binary)); }
                    if !field.decimal.is_empty() { ui.label(format!("10进制: {}", field.decimal)); }
                    if !field.hexadecimal.is_empty() { ui.label(format!("16进制: {}", field.hexadecimal)); }
                });
            }
        });
        ui.add_space(8.0);

        // Decimal converter
        ui.group(|ui| {
            ui.label(RichText::new("十进制转换").color(Color32::BLUE));
            
            let response = ui.add(
                TextEdit::singleline(&mut frontend.number_conversion.decimal_field.input)
                    .desired_width(300.0)
                    .hint_text("输入十进制数，如: 255"),
            );
            
            if response.changed() {
                frontend.request_decimal_conversion();
            }
            
            if frontend.number_conversion.decimal_field.pending_id.is_some() {
                ui.spinner();
            }
            
            let field = &frontend.number_conversion.decimal_field;
            if let Some(err) = &field.error {
                ui.colored_label(Color32::RED, err);
            } else if !field.binary.is_empty() || !field.decimal.is_empty() || !field.hexadecimal.is_empty() {
                ui.horizontal(|ui| {
                    if !field.binary.is_empty() { ui.label(format!("2进制: {}", field.binary)); }
                    if !field.decimal.is_empty() { ui.label(format!("10进制: {}", field.decimal)); }
                    if !field.hexadecimal.is_empty() { ui.label(format!("16进制: {}", field.hexadecimal)); }
                });
            }
        });
        ui.add_space(8.0);

        // Hex converter
        ui.group(|ui| {
            ui.label(RichText::new("十六进制转换").color(Color32::BLUE));
            
            let response = ui.add(
                TextEdit::singleline(&mut frontend.number_conversion.hex_field.input)
                    .desired_width(300.0)
                    .hint_text("输入十六进制数，如: FF"),
            );
            
            if response.changed() {
                frontend.request_hex_conversion();
            }
            
            if frontend.number_conversion.hex_field.pending_id.is_some() {
                ui.spinner();
            }
            
            let field = &frontend.number_conversion.hex_field;
            if let Some(err) = &field.error {
                ui.colored_label(Color32::RED, err);
            } else if !field.binary.is_empty() || !field.decimal.is_empty() || !field.hexadecimal.is_empty() {
                ui.horizontal(|ui| {
                    if !field.binary.is_empty() { ui.label(format!("2进制: {}", field.binary)); }
                    if !field.decimal.is_empty() { ui.label(format!("10进制: {}", field.decimal)); }
                    if !field.hexadecimal.is_empty() { ui.label(format!("16进制: {}", field.hexadecimal)); }
                });
            }
        });
        ui.add_space(8.0);

        ui.separator();
        ui.heading("浮点数转换");
        ui.add_space(10.0);

        // f32 to hex
        ui.group(|ui| {
            ui.label(RichText::new("f32 → 十六进制").color(Color32::BLUE));
            
            let response = ui.add(
                TextEdit::singleline(&mut frontend.float_conversion.f32_to_hex.input)
                    .desired_width(300.0)
                    .hint_text("输入f32数值，如: 1.0"),
            );
            
            if response.changed() {
                frontend.request_float_conversion(true);
            }
            
            if frontend.float_conversion.f32_to_hex.pending_id.is_some() {
                ui.spinner();
            }
            
            let field = &frontend.float_conversion.f32_to_hex;
            if let Some(err) = &field.error {
                ui.colored_label(Color32::RED, err);
            } else if !field.output.is_empty() {
                ui.label(format!("结果: {}", field.output));
            }
        });
        ui.add_space(8.0);

        // hex to f32
        ui.group(|ui| {
            ui.label(RichText::new("十六进制 → f32").color(Color32::BLUE));
            
            let response = ui.add(
                TextEdit::singleline(&mut frontend.float_conversion.hex_to_f32.input)
                    .desired_width(300.0)
                    .hint_text("输入8位十六进制，如: 3F800000"),
            );
            
            if response.changed() {
                frontend.request_float_conversion(false);
            }
            
            if frontend.float_conversion.hex_to_f32.pending_id.is_some() {
                ui.spinner();
            }
            
            let field = &frontend.float_conversion.hex_to_f32;
            if let Some(err) = &field.error {
                ui.colored_label(Color32::RED, err);
            } else if !field.output.is_empty() {
                ui.label(format!("结果: {}", field.output));
                if let Some(analysis) = &field.analysis {
                    ui.add_space(5.0);
                    egui::CollapsingHeader::new("IEEE 754 分析")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.monospace(analysis);
                        });
                }
            }
        });
        ui.add_space(8.0);

        // Action buttons
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("清除所有").clicked() {
                frontend.number_conversion.binary_field.input.clear();
                frontend.number_conversion.binary_field.binary.clear();
                frontend.number_conversion.binary_field.decimal.clear();
                frontend.number_conversion.binary_field.hexadecimal.clear();
                frontend.number_conversion.binary_field.error = None;
                frontend.number_conversion.binary_field.pending_id = None;
                
                frontend.number_conversion.decimal_field.input.clear();
                frontend.number_conversion.decimal_field.binary.clear();
                frontend.number_conversion.decimal_field.decimal.clear();
                frontend.number_conversion.decimal_field.hexadecimal.clear();
                frontend.number_conversion.decimal_field.error = None;
                frontend.number_conversion.decimal_field.pending_id = None;
                
                frontend.number_conversion.hex_field.input.clear();
                frontend.number_conversion.hex_field.binary.clear();
                frontend.number_conversion.hex_field.decimal.clear();
                frontend.number_conversion.hex_field.hexadecimal.clear();
                frontend.number_conversion.hex_field.error = None;
                frontend.number_conversion.hex_field.pending_id = None;
                
                frontend.float_conversion.f32_to_hex.input.clear();
                frontend.float_conversion.f32_to_hex.output.clear();
                frontend.float_conversion.f32_to_hex.error = None;
                frontend.float_conversion.f32_to_hex.pending_id = None;
                
                frontend.float_conversion.hex_to_f32.input.clear();
                frontend.float_conversion.hex_to_f32.output.clear();
                frontend.float_conversion.hex_to_f32.analysis = None;
                frontend.float_conversion.hex_to_f32.error = None;
                frontend.float_conversion.hex_to_f32.pending_id = None;
            }
            
            if ui.button("加载示例").clicked() {
                frontend.number_conversion.binary_field.input = "11111111".to_string();
                frontend.request_binary_conversion();
                
                frontend.number_conversion.decimal_field.input = "255".to_string();
                frontend.request_decimal_conversion();
                
                frontend.number_conversion.hex_field.input = "FF".to_string();
                frontend.request_hex_conversion();
                
                frontend.float_conversion.f32_to_hex.input = "3.14159".to_string();
                frontend.request_float_conversion(true);
                
                frontend.float_conversion.hex_to_f32.input = "40490FDB".to_string();
                frontend.request_float_conversion(false);
            }
        });
    });
}
