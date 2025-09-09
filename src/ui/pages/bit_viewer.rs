use eframe::egui::{self, Color32, RichText, TextEdit, Ui, Vec2, Sense, Align2, FontId, Stroke};
use crate::core::BitViewerData;

/// 位查看器页面
pub struct BitViewerPage {
    data: BitViewerData,
}

impl BitViewerPage {
    /// 创建新的位查看器页面
    pub fn new() -> Self {
        Self {
            data: BitViewerData::new(),
        }
    }

    /// 渲染页面
    pub fn render(&mut self, ui: &mut Ui) {
        // 固定的输入区域
        ui.horizontal(|ui| {
            ui.label(RichText::new("十六进制数据:").color(Color32::BLUE));
            let response = ui.add(
                TextEdit::singleline(self.data.hex_input_mut())
                    .desired_width(300.0)
                    .hint_text("输入十六进制数据，如: A1B2C3"),
            );

            if response.changed() {
                self.data.set_hex_input(self.data.hex_input().to_string());
            }

            // 操作按钮
            if ui.button("清除").clicked() {
                self.data.clear();
            }

            if ui.button("示例").clicked() {
                self.data.set_example();
            }
        });

        // 字段位数输入
        ui.horizontal(|ui| {
            ui.label(RichText::new("字段位数:").color(Color32::BLUE));
            let response = ui.add(
                TextEdit::singleline(self.data.field_widths_input_mut())
                    .desired_width(300.0)
                    .hint_text("输入字段位数，用空格分隔，如: 4 8 4"),
            );

            if response.changed() {
                self.data.set_field_widths_input(self.data.field_widths_input().to_string());
            }
        });

        ui.separator();

        // 错误显示
        if let Some(error) = self.data.last_error() {
            ui.colored_label(Color32::RED, error.to_string());
            return;
        }

        if self.data.binary_bits().is_empty() {
            return;
        }

        // 显示二进制位标题
        ui.label(RichText::new("二进制位 (从高位到低位):").color(Color32::DARK_GREEN));

        // 计算字段分组
        let field_groups = self.data.calculate_field_groups();

        // 计算可用的滚动区域高度
        let available_height = ui.available_height();
        let scroll_height = (available_height - 120.0).max(200.0);

        // 创建滚动区域显示位字段
        egui::ScrollArea::vertical()
            .max_height(scroll_height)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                self.display_bit_fields(ui, &field_groups);
                ui.separator();
                self.display_statistics(ui);
            });
    }

    /// 显示位字段
    fn display_bit_fields(&mut self, ui: &mut Ui, field_groups: &[usize]) {
        let mut bit_index = 0;
        let configured_fields_count = self.data.field_widths().len();

        for (field_index, &group_size) in field_groups.iter().enumerate() {
            if bit_index >= self.data.binary_bits().len() {
                break;
            }

            let field_start_bit = bit_index;
            let actual_group_size = group_size.min(self.data.binary_bits().len() - bit_index);

            // 显示字段标题和数值
            let field_title = if field_index < configured_fields_count {
                let field_value = self.calculate_field_value(field_start_bit, actual_group_size);
                format!("字段 {} ({} 位): 0x{:X} {}", 
                    field_index + 1, actual_group_size, field_value, field_value)
            } else {
                let field_value = self.calculate_field_value(field_start_bit, actual_group_size);
                format!("剩余位 ({} 位): 0x{:X} {}", 
                    actual_group_size, field_value, field_value)
            };

            ui.label(RichText::new(field_title).color(Color32::DARK_BLUE));

            // 显示该字段的位按钮
            ui.horizontal(|ui| {
                for _ in 0..actual_group_size {
                    if bit_index < self.data.binary_bits().len() {
                        let bit_value = self.data.binary_bits()[bit_index];
                        self.render_bit_button(ui, bit_index, bit_value);
                        bit_index += 1;
                    }
                }
            });

            // 显示位序号
            ui.horizontal(|ui| {
                let mut temp_bit_index = field_start_bit;
                for _ in 0..actual_group_size {
                    if temp_bit_index < self.data.binary_bits().len() {
                        let bit_position = self.data.binary_bits().len() - temp_bit_index - 1;
                        let (rect, _) = ui.allocate_exact_size(Vec2::new(24.0, 12.0), Sense::hover());
                        ui.painter().text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            format!("{}", bit_position),
                            FontId::monospace(8.0),
                            Color32::GRAY,
                        );
                        temp_bit_index += 1;
                    }
                }
            });

            ui.add_space(10.0);
        }
    }

    /// 渲染单个位按钮
    fn render_bit_button(&mut self, ui: &mut Ui, bit_index: usize, bit_value: bool) {
        let button_text = if bit_value { "1" } else { "0" };

        // 3D效果的颜色配置
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

            // 绘制按钮背景
            painter.rect_filled(
                rect.shrink(1.0),
                egui::CornerRadius::same(3),
                current_base_color,
            );

            // 绘制高光效果
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

            // 绘制阴影效果
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

            // 绘制边框
            painter.rect_stroke(
                rect.shrink(0.5),
                egui::CornerRadius::same(3),
                Stroke::new(1.0, Color32::from_gray(100)),
                egui::StrokeKind::Outside,
            );

            // 绘制文字
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                button_text,
                FontId::proportional(14.0),
                Color32::WHITE,
            );

            // 悬停效果
            if response.hovered() {
                painter.rect_stroke(
                    rect.shrink(-1.0),
                    egui::CornerRadius::same(4),
                    Stroke::new(1.5, Color32::from_rgb(255, 255, 100)),
                    egui::StrokeKind::Outside,
                );
            }
        }

        if response.clicked() {
            self.data.toggle_bit(bit_index);
        }
    }

    /// 显示统计信息
    fn display_statistics(&self, ui: &mut Ui) {
        ui.label(RichText::new("统计信息:").color(Color32::DARK_GREEN));
        
        let total_bits = self.data.binary_bits().len();
        let ones_count = self.data.binary_bits().iter().filter(|&&bit| bit).count();
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
                ui.label(format!("1的比例: {:.1}%", (ones_count as f32 / total_bits as f32) * 100.0));
                ui.separator();
                ui.label(format!("0的比例: {:.1}%", (zeros_count as f32 / total_bits as f32) * 100.0));
            });
        }
    }

    /// 计算指定字段的数值
    fn calculate_field_value(&self, start_bit: usize, bit_count: usize) -> u64 {
        let mut value = 0u64;
        for i in 0..bit_count {
            if start_bit + i < self.data.binary_bits().len() {
                if self.data.binary_bits()[start_bit + i] {
                    value |= 1 << (bit_count - 1 - i);
                }
            }
        }
        value
    }
}

impl Default for BitViewerPage {
    fn default() -> Self {
        Self::new()
    }
}


