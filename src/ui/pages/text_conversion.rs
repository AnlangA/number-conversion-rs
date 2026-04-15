//! Text conversion page.

use crate::frontend::FrontendState;
use crate::ui::components::{
    group_panel, helper_text, show_empty_state, show_error, show_instructions, show_pending,
};
use eframe::egui::{self, TextEdit, Ui};

/// Render the text conversion page.
pub fn render(ui: &mut Ui, frontend: &mut FrontendState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("文本转换");
        ui.add_space(10.0);

        // ASCII to Hex
        group_panel(ui, "ASCII → 十六进制", |ui| {
            let response = ui.add(
                TextEdit::singleline(&mut frontend.text_conversion.ascii_to_hex.input)
                    .desired_width(300.0)
                    .hint_text("输入 ASCII 文本，例如 Hello"),
            );

            if response.changed() {
                frontend.request_text_conversion(true);
            }

            if frontend.text_conversion.ascii_to_hex.pending_id.is_some() {
                show_pending(ui, "正在转换 ASCII 文本...");
            }

            let field = &frontend.text_conversion.ascii_to_hex;
            if !show_error(ui, field.error.as_deref()) {
                if !field.output.is_empty() {
                    ui.label("结果：");
                    ui.monospace(&field.output);
                } else {
                    show_empty_state(ui, "输入内容后，这里会显示对应的十六进制结果。");
                }
            }
        });
        ui.add_space(8.0);

        // Hex to ASCII
        group_panel(ui, "十六进制 → ASCII", |ui| {
            let response = ui.add(
                TextEdit::singleline(&mut frontend.text_conversion.hex_to_ascii.input)
                    .desired_width(300.0)
                    .hint_text("输入十六进制，例如 48 65 6C 6C 6F"),
            );

            if response.changed() {
                frontend.request_text_conversion(false);
            }

            if frontend.text_conversion.hex_to_ascii.pending_id.is_some() {
                show_pending(ui, "正在解析十六进制文本...");
            }

            let field = &frontend.text_conversion.hex_to_ascii;
            if !show_error(ui, field.error.as_deref()) {
                if !field.output.is_empty() {
                    ui.label("结果：");
                    ui.monospace(&field.output);
                } else {
                    show_empty_state(ui, "输入十六进制内容后，这里会显示对应的 ASCII 结果。");
                }
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

                frontend.text_conversion.hex_to_ascii.input =
                    "48 65 6C 6C 6F 20 57 6F 72 6C 64 21".to_string();
                frontend.request_text_conversion(false);
            }
        });

        // Usage instructions
        ui.separator();
        helper_text(ui, "支持实时转换，输入变化后会自动刷新结果。");
        show_instructions(
            ui,
            "使用说明",
            &[
                "• ASCII 转十六进制：将文本字符转换为对应的十六进制编码",
                "• 十六进制转 ASCII：将十六进制编码转换为对应的文本字符",
                "• 十六进制输入支持空格分隔，例如：48 65 6C 6C 6F",
                "• 不可打印字符将显示为 [0xXX] 格式",
            ],
        );
    });
}
