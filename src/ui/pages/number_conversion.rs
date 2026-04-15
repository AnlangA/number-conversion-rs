//! Number conversion page.

use crate::frontend::FrontendState;
use crate::ui::components::{
    group_panel, helper_text, labeled_monospace, show_empty_state, show_error, show_instructions,
    show_pending,
};
use eframe::egui::{self, TextEdit, Ui};

/// Render the number conversion page.
pub fn render(ui: &mut Ui, frontend: &mut FrontendState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("进制转换");
        ui.add_space(10.0);

        // Binary converter
        group_panel(ui, "二进制转换", |ui| {
            let response = ui.add(
                TextEdit::singleline(&mut frontend.number_conversion.binary_field.input)
                    .desired_width(300.0)
                    .hint_text("输入二进制数，例如 1010"),
            );

            if response.changed() {
                frontend.request_binary_conversion();
            }

            if frontend.number_conversion.binary_field.pending_id.is_some() {
                show_pending(ui, "正在转换二进制数值...");
            }

            let field = &frontend.number_conversion.binary_field;
            if !show_error(ui, field.error.as_deref()) {
                if !field.binary.is_empty()
                    || !field.decimal.is_empty()
                    || !field.hexadecimal.is_empty()
                {
                    if !field.binary.is_empty() {
                        labeled_monospace(ui, "二进制：", &field.binary);
                    }
                    if !field.decimal.is_empty() {
                        labeled_monospace(ui, "十进制：", &field.decimal);
                    }
                    if !field.hexadecimal.is_empty() {
                        labeled_monospace(ui, "十六进制：", &field.hexadecimal);
                    }
                } else {
                    show_empty_state(ui, "输入二进制内容后，这里会显示对应的多进制结果。");
                }
            }
        });
        ui.add_space(8.0);

        // Decimal converter
        group_panel(ui, "十进制转换", |ui| {
            let response = ui.add(
                TextEdit::singleline(&mut frontend.number_conversion.decimal_field.input)
                    .desired_width(300.0)
                    .hint_text("输入十进制数，例如 255"),
            );

            if response.changed() {
                frontend.request_decimal_conversion();
            }

            if frontend
                .number_conversion
                .decimal_field
                .pending_id
                .is_some()
            {
                show_pending(ui, "正在转换十进制数值...");
            }

            let field = &frontend.number_conversion.decimal_field;
            if !show_error(ui, field.error.as_deref()) {
                if !field.binary.is_empty()
                    || !field.decimal.is_empty()
                    || !field.hexadecimal.is_empty()
                {
                    if !field.binary.is_empty() {
                        labeled_monospace(ui, "二进制：", &field.binary);
                    }
                    if !field.decimal.is_empty() {
                        labeled_monospace(ui, "十进制：", &field.decimal);
                    }
                    if !field.hexadecimal.is_empty() {
                        labeled_monospace(ui, "十六进制：", &field.hexadecimal);
                    }
                } else {
                    show_empty_state(ui, "输入十进制内容后，这里会显示对应的多进制结果。");
                }
            }
        });
        ui.add_space(8.0);

        // Hex converter
        group_panel(ui, "十六进制转换", |ui| {
            let response = ui.add(
                TextEdit::singleline(&mut frontend.number_conversion.hex_field.input)
                    .desired_width(300.0)
                    .hint_text("输入十六进制数，例如 FF"),
            );

            if response.changed() {
                frontend.request_hex_conversion();
            }

            if frontend.number_conversion.hex_field.pending_id.is_some() {
                show_pending(ui, "正在转换十六进制数值...");
            }

            let field = &frontend.number_conversion.hex_field;
            if !show_error(ui, field.error.as_deref()) {
                if !field.binary.is_empty()
                    || !field.decimal.is_empty()
                    || !field.hexadecimal.is_empty()
                {
                    if !field.binary.is_empty() {
                        labeled_monospace(ui, "二进制：", &field.binary);
                    }
                    if !field.decimal.is_empty() {
                        labeled_monospace(ui, "十进制：", &field.decimal);
                    }
                    if !field.hexadecimal.is_empty() {
                        labeled_monospace(ui, "十六进制：", &field.hexadecimal);
                    }
                } else {
                    show_empty_state(ui, "输入十六进制内容后，这里会显示对应的多进制结果。");
                }
            }
        });
        ui.add_space(8.0);

        ui.separator();
        ui.heading("浮点数转换");
        ui.add_space(10.0);

        // f32 to hex
        group_panel(ui, "f32 → 十六进制", |ui| {
            let response = ui.add(
                TextEdit::singleline(&mut frontend.float_conversion.f32_to_hex.input)
                    .desired_width(300.0)
                    .hint_text("输入 f32 数值，例如 1.0"),
            );

            if response.changed() {
                frontend.request_float_conversion(true);
            }

            if frontend.float_conversion.f32_to_hex.pending_id.is_some() {
                show_pending(ui, "正在转换浮点数...");
            }

            let field = &frontend.float_conversion.f32_to_hex;
            if !show_error(ui, field.error.as_deref()) {
                if !field.output.is_empty() {
                    labeled_monospace(ui, "结果：", &field.output);
                } else {
                    show_empty_state(ui, "输入浮点数后，这里会显示对应的十六进制结果。");
                }
            }
        });
        ui.add_space(8.0);

        // hex to f32
        group_panel(ui, "十六进制 → f32", |ui| {
            let response = ui.add(
                TextEdit::singleline(&mut frontend.float_conversion.hex_to_f32.input)
                    .desired_width(300.0)
                    .hint_text("输入 8 位十六进制，例如 3F800000"),
            );

            if response.changed() {
                frontend.request_float_conversion(false);
            }

            if frontend.float_conversion.hex_to_f32.pending_id.is_some() {
                show_pending(ui, "正在解析浮点十六进制表示...");
            }

            let field = &frontend.float_conversion.hex_to_f32;
            if !show_error(ui, field.error.as_deref()) {
                if !field.output.is_empty() {
                    labeled_monospace(ui, "结果：", &field.output);
                    if let Some(analysis) = &field.analysis {
                        ui.add_space(5.0);
                        egui::CollapsingHeader::new("IEEE 754 分析")
                            .default_open(false)
                            .show(ui, |ui| {
                                ui.monospace(analysis);
                            });
                    }
                } else {
                    show_empty_state(ui, "输入十六进制浮点表示后，这里会显示对应的 f32 结果。");
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

        ui.separator();
        helper_text(ui, "所有输入框均支持实时转换，输入变化后会自动刷新结果。");
        show_instructions(
            ui,
            "使用说明",
            &[
                "• 二进制、十进制、十六进制输入会实时互相转换",
                "• 浮点数转换遵循 IEEE 754 单精度（f32）格式",
                "• 十六进制转 f32 需要输入 8 位十六进制数值",
                "• 点击“加载示例”可快速查看典型输入输出效果",
            ],
        );
    });
}
