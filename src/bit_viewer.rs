use crate::data::*;
use eframe::egui;
use egui::*;

pub struct BitViewerData {
    pub hex_input: String,
    pub field_widths: String,
    pub binary_bits: Vec<bool>,
    pub parsed_field_widths: Vec<usize>,
    pub data_error: DataError,
}

impl BitViewerData {
    pub fn new() -> Self {
        Self {
            hex_input: String::new(),
            field_widths: "4 4 4 4 4 4 4 4".to_string(), // 默认字段分割为4位
            binary_bits: Vec::new(),
            parsed_field_widths: vec![4, 4, 4, 4, 4, 4, 4, 4], // 默认解析为4位字段
            data_error: DataError::Nice,
        }
    }

    pub fn update_from_hex(&mut self) {
        self.data_error = DataError::Nice;
        
        // 清理十六进制输入（移除空格和下划线）
        let clean_hex = self.hex_input
            .replace(" ", "")
            .replace("_", "")
            .to_uppercase();

        if clean_hex.is_empty() {
            self.data_error = DataError::LenNull;
            self.binary_bits.clear();
            return;
        }

        // 验证十六进制格式
        if !clean_hex.chars().all(|c| c.is_ascii_hexdigit()) {
            self.data_error = DataError::FormatError;
            self.binary_bits.clear();
            return;
        }

        // 转换为二进制位
        self.binary_bits.clear();
        for hex_char in clean_hex.chars() {
            let digit = hex_char.to_digit(16).unwrap() as u8;
            for i in (0..4).rev() {
                self.binary_bits.push((digit & (1 << i)) != 0);
            }
        }
    }

    pub fn parse_field_widths(&mut self) {
        self.parsed_field_widths.clear();
        
        if self.field_widths.trim().is_empty() {
            return;
        }

        for width_str in self.field_widths.split_whitespace() {
            if let Ok(width) = width_str.parse::<usize>() {
                if width > 0 {
                    self.parsed_field_widths.push(width);
                }
            }
        }
    }

    pub fn toggle_bit(&mut self, index: usize) {
        if index < self.binary_bits.len() {
            self.binary_bits[index] = !self.binary_bits[index];
            self.update_hex_from_bits();
        }
    }

    fn update_hex_from_bits(&mut self) {
        if self.binary_bits.is_empty() {
            self.hex_input.clear();
            return;
        }

        let mut hex_string = String::new();
        
        // 确保位数是4的倍数（补零）
        let mut bits = self.binary_bits.clone();
        while bits.len() % 4 != 0 {
            bits.insert(0, false);
        }

        // 每4位转换为一个十六进制字符
        for chunk in bits.chunks(4) {
            let mut value = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit {
                    value |= 1 << (3 - i);
                }
            }
            hex_string.push_str(&format!("{:X}", value));
        }

        self.hex_input = hex_string;
    }
}

pub fn bit_viewer(data: &mut BitViewerData, ui: &mut Ui) {

    // 固定的输入区域（不滚动）
    ui.horizontal(|ui| {
        ui.label(RichText::from("十六进制数据:").color(Color32::BLUE));
        let response = ui.add(
            TextEdit::singleline(&mut data.hex_input)
                .desired_width(300.0)
                .hint_text("输入十六进制数据，如: A1B2C3")
        );

        if response.changed() {
            data.update_from_hex();
        }
    });

    // 字段位数输入
    ui.horizontal(|ui| {
        ui.label(RichText::from("字段位数:").color(Color32::BLUE));
        let response = ui.add(
            TextEdit::singleline(&mut data.field_widths)
                .desired_width(300.0)
                .hint_text("输入字段位数，用空格分隔，如: 4 8 4")
        );

        if response.changed() {
            data.parse_field_widths();
        }
    });

    ui.separator();

    // 错误显示
    match data.data_error {
        DataError::LenNull => {
            ui.colored_label(Color32::RED, "请输入十六进制数据");
            return;
        }
        DataError::FormatError => {
            ui.colored_label(Color32::RED, "请输入有效的十六进制字符");
            return;
        }
        _ => {}
    }

    if data.binary_bits.is_empty() {
        return;
    }

    // 显示二进制位标题
    ui.label(RichText::from("二进制位 (从高位到低位):").color(Color32::DARK_GREEN));

    // 计算字段分组
    let field_groups = calculate_field_groups(&data.binary_bits, &data.parsed_field_widths);

    // 创建滚动区域来显示所有字段
    egui::ScrollArea::vertical()
        .max_height(400.0) // 设置最大高度，超过后显示滚动条
        .auto_shrink([false; 2]) // 不自动收缩
        .show(ui, |ui| {
            // 在滚动区域内显示所有字段
            display_bit_fields(data, ui, &field_groups);
        });

    ui.separator();

    // 统计信息（固定在底部，不滚动）
    display_statistics(data, ui);
}

fn display_bit_fields(data: &mut BitViewerData, ui: &mut Ui, field_groups: &[usize]) {
    let mut bit_index = 0;

    for (group_index, group_size) in field_groups.iter().enumerate() {
        if group_index > 0 {
            ui.add_space(10.0);
        }

        // 显示字段标签和十六进制值
        ui.horizontal(|ui| {
            if !data.parsed_field_widths.is_empty() {
                ui.label(RichText::from(format!("字段{}({} bits)", group_index + 1, group_size))
                    .color(Color32::GRAY)
                    .size(10.0));
            }

            // 添加一些间距，但不要太远
            ui.add_space(20.0);

            // 计算该字段的十六进制值
            let field_hex_value = calculate_field_hex_value(&data.binary_bits, bit_index, *group_size);
            ui.label(RichText::from(format!("0x{}", field_hex_value))
                .color(Color32::BLUE)
                .size(12.0));
        });

        let field_start_bit = bit_index;

        // 显示该字段的位按钮
        ui.horizontal(|ui| {
            for _ in 0..*group_size {
                if bit_index < data.binary_bits.len() {
                    let bit_value = data.binary_bits[bit_index];
                    let button_text = if bit_value { "1" } else { "0" };

                    // 3D效果的颜色配置
                    let (base_color, highlight_color, shadow_color) = if bit_value {
                        // 绿色系 - 表示1
                        (
                            Color32::from_rgb(80, 180, 80),   // 基础绿色
                            Color32::from_rgb(120, 220, 120), // 高光绿色
                            Color32::from_rgb(40, 120, 40),   // 阴影绿色
                        )
                    } else {
                        // 红色系 - 表示0
                        (
                            Color32::from_rgb(180, 80, 80),   // 基础红色
                            Color32::from_rgb(220, 120, 120), // 高光红色
                            Color32::from_rgb(120, 40, 40),   // 阴影红色
                        )
                    };

                    // 创建3D效果的按钮 - 缩小尺寸
                    let button_size = Vec2::new(24.0, 24.0);
                    let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());

                    if ui.is_rect_visible(rect) {
                        let painter = ui.painter();

                        // 根据按钮状态选择颜色
                        let current_base_color = if response.is_pointer_button_down_on() {
                            shadow_color // 按下时使用阴影色
                        } else if response.hovered() {
                            highlight_color // 悬停时使用高光色
                        } else {
                            base_color // 正常状态
                        };

                        // 绘制按钮背景
                        painter.rect_filled(
                            rect.shrink(1.0),
                            Rounding::same(3.0), // 缩小圆角
                            current_base_color,
                        );

                        // 绘制高光效果（顶部和左侧）
                        if !response.is_pointer_button_down_on() {
                            // 顶部高光
                            painter.line_segment(
                                [rect.left_top() + Vec2::new(2.0, 2.0), rect.right_top() + Vec2::new(-2.0, 2.0)],
                                Stroke::new(1.5, highlight_color),
                            );
                            // 左侧高光
                            painter.line_segment(
                                [rect.left_top() + Vec2::new(2.0, 2.0), rect.left_bottom() + Vec2::new(2.0, -2.0)],
                                Stroke::new(1.5, highlight_color),
                            );
                        }

                        // 绘制阴影效果（底部和右侧）
                        painter.line_segment(
                            [rect.right_top() + Vec2::new(-2.0, 2.0), rect.right_bottom() + Vec2::new(-2.0, -2.0)],
                            Stroke::new(1.5, shadow_color),
                        );
                        painter.line_segment(
                            [rect.left_bottom() + Vec2::new(2.0, -2.0), rect.right_bottom() + Vec2::new(-2.0, -2.0)],
                            Stroke::new(1.5, shadow_color),
                        );

                        // 绘制边框
                        painter.rect_stroke(
                            rect.shrink(0.5),
                            Rounding::same(3.0), // 缩小圆角
                            Stroke::new(1.0, Color32::from_gray(100)),
                        );

                        // 绘制文字
                        let text_color = if bit_value { Color32::WHITE } else { Color32::WHITE };
                        painter.text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            button_text,
                            FontId::proportional(14.0), // 缩小字体
                            text_color,
                        );

                        // 悬停效果
                        if response.hovered() {
                            painter.rect_stroke(
                                rect.shrink(-1.0),
                                Rounding::same(4.0), // 缩小圆角
                                Stroke::new(1.5, Color32::from_rgb(255, 255, 100)), // 缩小边框宽度
                            );
                        }
                    }

                    if response.clicked() {
                        data.toggle_bit(bit_index);
                    }

                    bit_index += 1;
                }
            }
        });

        // 显示位序号
        ui.horizontal(|ui| {
            let mut temp_bit_index = field_start_bit;
            for _ in 0..*group_size {
                if temp_bit_index < data.binary_bits.len() {
                    // 计算位序号（从高位开始，所以是总位数减去当前索引减1）
                    let bit_position = data.binary_bits.len() - temp_bit_index - 1;

                    // 创建与按钮相同大小的区域来确保对齐
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(24.0, 12.0), Sense::hover());

                    // 在区域中心绘制位序号
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
    }
}

fn display_statistics(data: &BitViewerData, ui: &mut Ui) {
    // 显示统计信息
    ui.horizontal(|ui| {
        ui.label(RichText::from("总位数:").color(Color32::GRAY));
        ui.monospace(format!("{}", data.binary_bits.len()));

        ui.separator();

        ui.label(RichText::from("十六进制长度:").color(Color32::GRAY));
        ui.monospace(format!("{} 字符", data.hex_input.len()));

        if !data.parsed_field_widths.is_empty() {
            ui.separator();
            ui.label(RichText::from("字段数:").color(Color32::GRAY));
            ui.monospace(format!("{}", data.parsed_field_widths.len()));
        }
    });
}

fn calculate_field_groups(bits: &[bool], field_widths: &[usize]) -> Vec<usize> {
    if field_widths.is_empty() {
        return vec![bits.len()];
    }

    let mut groups = Vec::new();
    let mut remaining_bits = bits.len();

    // 从低位开始分配字段（但显示时从高位开始）
    for &width in field_widths.iter() {
        if remaining_bits == 0 {
            break;
        }

        let actual_width = width.min(remaining_bits);
        groups.push(actual_width);
        remaining_bits -= actual_width;
    }

    // 如果还有剩余位，作为最后一个字段
    if remaining_bits > 0 {
        groups.push(remaining_bits);
    }

    // 反转顺序，因为我们要从高位开始显示
    groups.reverse();
    groups
}

fn calculate_field_hex_value(bits: &[bool], start_index: usize, field_size: usize) -> String {
    if start_index >= bits.len() {
        return "0".to_string();
    }

    let end_index = (start_index + field_size).min(bits.len());
    let field_bits = &bits[start_index..end_index];

    if field_bits.is_empty() {
        return "0".to_string();
    }

    // 将位转换为数值
    let mut value = 0u64;
    for (i, &bit) in field_bits.iter().enumerate() {
        if bit {
            // 从高位开始，所以第一个位是最高位
            value |= 1u64 << (field_bits.len() - 1 - i);
        }
    }

    // 根据位数选择合适的格式
    if field_size <= 4 {
        format!("{:X}", value)
    } else if field_size <= 8 {
        format!("{:02X}", value)
    } else if field_size <= 16 {
        format!("{:04X}", value)
    } else {
        format!("{:X}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_binary() {
        let mut data = BitViewerData::new();
        data.hex_input = "A1".to_string();
        data.update_from_hex();
        
        // A1 = 10100001
        let expected = vec![true, false, true, false, false, false, false, true];
        assert_eq!(data.binary_bits, expected);
    }

    #[test]
    fn test_binary_to_hex() {
        let mut data = BitViewerData::new();
        data.binary_bits = vec![true, false, true, false, false, false, false, true];
        data.update_hex_from_bits();
        
        assert_eq!(data.hex_input, "A1");
    }

    #[test]
    fn test_toggle_bit() {
        let mut data = BitViewerData::new();
        data.hex_input = "A0".to_string();
        data.update_from_hex();
        
        // 切换最后一位
        data.toggle_bit(7);
        
        assert_eq!(data.hex_input, "A1");
    }

    #[test]
    fn test_field_width_parsing() {
        let mut data = BitViewerData::new();
        data.field_widths = "4 8 4".to_string();
        data.parse_field_widths();

        assert_eq!(data.parsed_field_widths, vec![4, 8, 4]);
    }

    #[test]
    fn test_default_field_width() {
        let data = BitViewerData::new();

        // 测试默认字段分割为4位（8个字段）
        assert_eq!(data.field_widths, "4 4 4 4 4 4 4 4");
        assert_eq!(data.parsed_field_widths, vec![4, 4, 4, 4, 4, 4, 4, 4]);
    }

    #[test]
    fn test_calculate_field_hex_value() {
        // 测试4位字段：1010 = A
        let bits = vec![true, false, true, false];
        let result = calculate_field_hex_value(&bits, 0, 4);
        assert_eq!(result, "A");

        // 测试8位字段：10100001 = A1
        let bits = vec![true, false, true, false, false, false, false, true];
        let result = calculate_field_hex_value(&bits, 0, 8);
        assert_eq!(result, "A1");

        // 测试部分字段
        let bits = vec![true, true, false, false, true, true, false, false];
        let result = calculate_field_hex_value(&bits, 0, 4); // 前4位：1100 = C
        assert_eq!(result, "C");

        let result = calculate_field_hex_value(&bits, 4, 4); // 后4位：1100 = C
        assert_eq!(result, "C");
    }

    #[test]
    fn test_calculate_field_hex_value_edge_cases() {
        // 测试空位
        let bits = vec![];
        let result = calculate_field_hex_value(&bits, 0, 4);
        assert_eq!(result, "0");

        // 测试超出范围
        let bits = vec![true, false];
        let result = calculate_field_hex_value(&bits, 5, 4);
        assert_eq!(result, "0");

        // 测试单位
        let bits = vec![true];
        let result = calculate_field_hex_value(&bits, 0, 1);
        assert_eq!(result, "1");

        let bits = vec![false];
        let result = calculate_field_hex_value(&bits, 0, 1);
        assert_eq!(result, "0");
    }
}
