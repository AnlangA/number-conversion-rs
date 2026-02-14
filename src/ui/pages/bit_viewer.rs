//! Bit viewer page.

use eframe::egui::{self, Align2, Color32, FontId, RichText, Sense, Stroke, Ui, TextEdit, Vec2};
use crate::frontend::FrontendState;

/// Render the bit viewer page.
pub fn render(ui: &mut Ui, frontend: &mut FrontendState) {
    // Input area
    ui.horizontal(|ui| {
        ui.label(RichText::new("十六进制数据:").color(Color32::BLUE));
        let response = ui.add(
            TextEdit::singleline(&mut frontend.bit_viewer.hex_input)
                .desired_width(300.0)
                .hint_text("输入十六进制数据，如: A1B2C3"),
        );

        if response.changed() {
            frontend.request_bit_viewer_parse();
        }

        if ui.button("清除").clicked() {
            frontend.bit_viewer.hex_input.clear();
            frontend.bit_viewer.binary_bits.clear();
            frontend.bit_viewer.error = None;
        }

        if ui.button("示例").clicked() {
            frontend.bit_viewer.hex_input = "A1B2C3D4".to_string();
            frontend.request_bit_viewer_parse();
        }

        // Invert button
        let invert_enabled = !frontend.bit_viewer.binary_bits.is_empty();
        let invert_button = ui
            .add_enabled(invert_enabled, egui::Button::new("按位取反"))
            .on_hover_text("将所有位进行按位取反操作 (0→1, 1→0)");

        if invert_button.clicked() {
            frontend.request_bit_invert_all();
        }
    });

    // Field width input
    ui.horizontal(|ui| {
        ui.label(RichText::new("字段位数:").color(Color32::BLUE));
        let response = ui.add(
            TextEdit::singleline(&mut frontend.bit_viewer.field_widths_input)
                .desired_width(300.0)
                .hint_text("输入字段位数，用空格分隔，如: 4 8 4"),
        );

        if response.changed() {
            frontend.bit_viewer.parse_field_widths();
        }
    });

    // Pending indicator
    if frontend.bit_viewer.pending_id.is_some() {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label("处理中...");
        });
    }

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
    ui.label(RichText::new("二进制位 (从高位到低位):").color(Color32::DARK_GREEN));

    // Calculate field groups
    let field_groups = frontend.bit_viewer.calculate_field_groups();

    // Calculate scroll area height
    let available_height = ui.available_height();
    let scroll_height = (available_height - 120.0).max(200.0);

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
            let field_value = frontend.bit_viewer.calculate_field_value(field_start_bit, actual_group_size);
            format!(
                "字段 {} ({} 位): 0x{:X} {}",
                field_index + 1,
                actual_group_size,
                field_value,
                field_value
            )
        } else {
            let field_value = frontend.bit_viewer.calculate_field_value(field_start_bit, actual_group_size);
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
    ui.label(RichText::new("统计信息:").color(Color32::DARK_GREEN));

    let total_bits = bits.len();
    let ones_count = bits.iter().filter(|&&bit| bit).count();
    let zeros_count = total_bits - ones_count;

    ui.horizontal(|ui| {
        ui.label(format!("总位数: {}", total_bits));
        ui.separator();
        ui.label(format!("1的个数: {}", ones_count));
        ui.separator();
        ui.label(format!("0的个数: {}", zeros_count));
    });

    if total_bits > 0 {
        ui.horizontal(|ui| {
            ui.label(format!(
                "1的比例: {:.1}%",
                (ones_count as f32 / total_bits as f32) * 100.0
            ));
            ui.separator();
            ui.label(format!(
                "0的比例: {:.1}%",
                (zeros_count as f32 / total_bits as f32) * 100.0
            ));
        });
    }
}
