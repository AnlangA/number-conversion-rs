//! Bit viewer page.

use crate::frontend::FrontendState;
use eframe::egui::{self, Align2, Color32, FontId, RichText, Sense, Stroke, TextEdit, Ui, Vec2};

/// Render the bit viewer page.
pub fn render(ui: &mut Ui, frontend: &mut FrontendState) {
    let has_bits = frontend.bit_viewer.has_bits();
    let shift_mode_label = if frontend.bit_viewer.zero_fill_shift {
        "逻辑移位"
    } else {
        "循环移位"
    };

    // Input area
    ui.horizontal(|ui| {
        ui.label(RichText::new("十六进制输入:").color(Color32::BLUE));
        let response = ui.add(
            TextEdit::singleline(&mut frontend.bit_viewer.hex_input)
                .desired_width(300.0)
                .hint_text("输入十六进制数据，例如 A1B2C3D4"),
        );

        if response.changed() {
            frontend.request_bit_viewer_parse();
        }

        if ui.button("清除").clicked() {
            frontend.bit_viewer.clear_data();
        }

        if ui.button("示例").clicked() {
            frontend.bit_viewer.hex_input = "A1B2C3D4".to_string();
            frontend.request_bit_viewer_parse();
        }

        let undo_button = ui
            .add_enabled(frontend.bit_viewer.can_undo(), egui::Button::new("撤销"))
            .on_hover_text("恢复到上一次位操作之前的状态");

        if undo_button.clicked() {
            frontend.request_bit_undo();
        }

        let redo_button = ui
            .add_enabled(frontend.bit_viewer.can_redo(), egui::Button::new("重做"))
            .on_hover_text("重新应用刚刚撤销的位操作");

        if redo_button.clicked() {
            frontend.request_bit_redo();
        }

        let invert_button = ui
            .add_enabled(has_bits, egui::Button::new("按位取反"))
            .on_hover_text("将当前所有位翻转：0 变 1，1 变 0");

        if invert_button.clicked() {
            frontend.request_bit_invert_all();
        }
    });

    ui.horizontal(|ui| {
        ui.label(RichText::new("位串输入:").color(Color32::BLUE));
        let response = ui.add(
            TextEdit::singleline(&mut frontend.bit_viewer.bit_string)
                .desired_width(420.0)
                .hint_text("直接输入位串，例如 10101100"),
        );

        if response.changed() {
            frontend.request_bit_string_parse();
        }
    });

    ui.horizontal(|ui| {
        ui.label(RichText::new("十进制输入:").color(Color32::BLUE));
        let response = ui.add(
            TextEdit::singleline(&mut frontend.bit_viewer.decimal_input)
                .desired_width(220.0)
                .hint_text("输入无符号十进制数值"),
        );

        if response.changed() {
            frontend.request_bit_decimal_parse();
        }

        if has_bits {
            ui.separator();
            ui.label(
                RichText::new(format!(
                    "当前位宽：{} 位，十进制值：{}",
                    frontend.bit_viewer.binary_bits.len(),
                    frontend.bit_viewer.decimal_input
                ))
                .color(Color32::DARK_GRAY),
            );
        }
    });

    // Field width input
    ui.horizontal(|ui| {
        ui.label(RichText::new("字段分组:").color(Color32::BLUE));
        let response = ui.add(
            TextEdit::singleline(&mut frontend.bit_viewer.field_widths_input)
                .desired_width(300.0)
                .hint_text("使用空格分隔每组位数，例如 4 8 4"),
        );

        if response.changed() {
            frontend.bit_viewer.parse_field_widths();
        }
    });

    // Shift controls
    ui.horizontal(|ui| {
        ui.label(RichText::new("移位设置:").color(Color32::BLUE));
        let response = ui.add(
            TextEdit::singleline(&mut frontend.bit_viewer.shift_count_input)
                .desired_width(80.0)
                .hint_text("位数"),
        );

        if response.changed() {
            frontend.bit_viewer.parse_shift_count();
        }

        ui.checkbox(&mut frontend.bit_viewer.zero_fill_shift, "超出位清零")
            .on_hover_text("开启后使用逻辑移位，移出范围的位会被丢弃并补 0；关闭后使用循环移位");

        let left_shift_button = ui
            .add_enabled(has_bits, egui::Button::new("左移"))
            .on_hover_text(if frontend.bit_viewer.zero_fill_shift {
                "向左移动指定位数，右侧空位补 0"
            } else {
                "向左循环移动指定位数，移出的高位回到低位"
            });

        if left_shift_button.clicked() {
            frontend.request_bit_shift_left();
        }

        let right_shift_button = ui
            .add_enabled(has_bits, egui::Button::new("右移"))
            .on_hover_text(if frontend.bit_viewer.zero_fill_shift {
                "向右移动指定位数，左侧空位补 0"
            } else {
                "向右循环移动指定位数，移出的低位回到高位"
            });

        if right_shift_button.clicked() {
            frontend.request_bit_shift_right();
        }

        if has_bits {
            ui.separator();
            ui.label(
                RichText::new(format!(
                    "当前共 {} 位，本次移动 {} 位，模式：{}",
                    frontend.bit_viewer.binary_bits.len(),
                    frontend.bit_viewer.shift_count,
                    shift_mode_label
                ))
                .color(Color32::DARK_GRAY),
            );
        }
    });

    ui.separator();

    // Error display
    if let Some(error) = &frontend.bit_viewer.error {
        ui.colored_label(Color32::RED, error);
        return;
    }

    if frontend.bit_viewer.binary_bits.is_empty() {
        return;
    }

    // Display title
    ui.label(RichText::new("位图预览（从高位到低位）:").color(Color32::DARK_GREEN));

    // Calculate field groups
    let field_groups = frontend.bit_viewer.calculate_field_groups();

    // Calculate scroll area height
    let available_height = ui.available_height();
    let scroll_height = (available_height - 150.0).max(220.0);

    // Display bit fields
    egui::ScrollArea::vertical()
        .max_height(scroll_height)
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            display_bit_fields(ui, frontend, &field_groups);
            ui.separator();
            display_statistics(ui, &frontend.bit_viewer.binary_bits);
        });
}

fn display_bit_fields(ui: &mut Ui, frontend: &mut FrontendState, field_groups: &[usize]) {
    let mut bit_index = 0;
    let configured_fields_count = frontend.bit_viewer.field_widths.len();

    for (field_index, &group_size) in field_groups.iter().enumerate() {
        if bit_index >= frontend.bit_viewer.binary_bits.len() {
            break;
        }

        let field_start_bit = bit_index;
        let actual_group_size = group_size.min(frontend.bit_viewer.binary_bits.len() - bit_index);

        // Update bit_index for next iteration
        bit_index += actual_group_size;

        // Display field title and value
        let field_title = if field_index < configured_fields_count {
            let field_value = frontend
                .bit_viewer
                .calculate_field_value(field_start_bit, actual_group_size);
            format!(
                "字段 {} ({} 位): 0x{:X} {}",
                field_index + 1,
                actual_group_size,
                field_value,
                field_value
            )
        } else {
            let field_value = frontend
                .bit_viewer
                .calculate_field_value(field_start_bit, actual_group_size);
            format!(
                "剩余位 ({} 位): 0x{:X} {}",
                actual_group_size, field_value, field_value
            )
        };

        ui.label(RichText::new(field_title).color(Color32::DARK_BLUE));

        // Display bit buttons
        ui.horizontal(|ui| {
            for i in 0..actual_group_size {
                let idx = field_start_bit + i;
                if idx < frontend.bit_viewer.binary_bits.len() {
                    let bit_value = frontend.bit_viewer.binary_bits[idx];
                    if render_bit_button(ui, idx, bit_value).clicked() {
                        frontend.request_bit_toggle(idx);
                    }
                }
            }
        });

        // Display bit positions
        ui.horizontal(|ui| {
            for i in 0..actual_group_size {
                let idx = field_start_bit + i;
                if idx < frontend.bit_viewer.binary_bits.len() {
                    let bit_position = frontend.bit_viewer.binary_bits.len() - idx - 1;
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(24.0, 12.0), Sense::hover());
                    ui.painter().text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        format!("{}", bit_position),
                        FontId::monospace(8.0),
                        Color32::GRAY,
                    );
                }
            }
        });

        ui.add_space(10.0);
    }
}

fn render_bit_button(ui: &mut Ui, _bit_index: usize, bit_value: bool) -> egui::Response {
    let button_text = if bit_value { "1" } else { "0" };

    let (base_color, highlight_color, shadow_color) = if bit_value {
        (
            Color32::from_rgb(80, 180, 80),
            Color32::from_rgb(120, 220, 120),
            Color32::from_rgb(40, 120, 40),
        )
    } else {
        (
            Color32::from_rgb(180, 80, 80),
            Color32::from_rgb(220, 120, 120),
            Color32::from_rgb(120, 40, 40),
        )
    };

    let button_size = Vec2::new(24.0, 24.0);
    let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        let current_base_color = if response.is_pointer_button_down_on() {
            shadow_color
        } else if response.hovered() {
            highlight_color
        } else {
            base_color
        };

        // Draw button background
        painter.rect_filled(
            rect.shrink(1.0),
            egui::CornerRadius::same(3),
            current_base_color,
        );

        // Draw highlight
        if !response.is_pointer_button_down_on() {
            painter.line_segment(
                [
                    rect.left_top() + Vec2::new(2.0, 2.0),
                    rect.right_top() + Vec2::new(-2.0, 2.0),
                ],
                Stroke::new(1.5, highlight_color),
            );
            painter.line_segment(
                [
                    rect.left_top() + Vec2::new(2.0, 2.0),
                    rect.left_bottom() + Vec2::new(2.0, -2.0),
                ],
                Stroke::new(1.5, highlight_color),
            );
        }

        // Draw shadow
        painter.line_segment(
            [
                rect.right_top() + Vec2::new(-2.0, 2.0),
                rect.right_bottom() + Vec2::new(-2.0, -2.0),
            ],
            Stroke::new(1.5, shadow_color),
        );
        painter.line_segment(
            [
                rect.left_bottom() + Vec2::new(2.0, -2.0),
                rect.right_bottom() + Vec2::new(-2.0, -2.0),
            ],
            Stroke::new(1.5, shadow_color),
        );

        // Draw border
        painter.rect_stroke(
            rect.shrink(0.5),
            egui::CornerRadius::same(3),
            Stroke::new(1.0, Color32::from_gray(100)),
            egui::StrokeKind::Outside,
        );

        // Draw text
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            button_text,
            FontId::proportional(14.0),
            Color32::WHITE,
        );

        // Hover effect
        if response.hovered() {
            painter.rect_stroke(
                rect.shrink(-1.0),
                egui::CornerRadius::same(4),
                Stroke::new(1.5, Color32::from_rgb(255, 255, 100)),
                egui::StrokeKind::Outside,
            );
        }
    }

    response
}

fn display_statistics(ui: &mut Ui, bits: &[bool]) {
    ui.label(
        RichText::new("统计信息:")
            .color(Color32::DARK_GREEN)
            .strong(),
    );

    let total_bits = bits.len();
    let ones_count = bits.iter().filter(|&&bit| bit).count();
    let zeros_count = total_bits - ones_count;

    ui.horizontal(|ui| {
        ui.label(format!("总位数：{}", total_bits));
        ui.separator();
        ui.label(format!("1 的个数：{}", ones_count));
        ui.separator();
        ui.label(format!("0 的个数：{}", zeros_count));
    });

    if total_bits > 0 {
        ui.horizontal(|ui| {
            ui.label(format!(
                "1 的占比：{:.1}%",
                (ones_count as f32 / total_bits as f32) * 100.0
            ));
            ui.separator();
            ui.label(format!(
                "0 的占比：{:.1}%",
                (zeros_count as f32 / total_bits as f32) * 100.0
            ));
        });
    }
}
