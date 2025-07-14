use crate::data::*;
use eframe::egui;
use egui::*;

pub fn ascii_hex(data: &mut Data, ui: &mut Ui) {
    data.set_data_error(DataError::Nice);
    let mut input_data = String::new();

    ui.horizontal(|ui| {
        ui.label(RichText::from("ASCII转Hex").color(Color32::BLUE))
            .on_hover_text("输入ASCII文本，自动转换为十六进制字符串");
        let text_edit = TextEdit::singleline(&mut data.input_data).desired_width(400.0);
        ui.add(text_edit);

        input_data = data.ref_input_data().clone();

        if input_data.is_empty() {
            data.set_data_error(DataError::LenNull);
        }
    });

    ui.horizontal(|ui| {
        match data.get_data_error() {
            DataError::LenNull => {
                ui.colored_label(Color32::RED, "请输入ASCII文本");
            }
            DataError::LenOver => {
                ui.colored_label(Color32::RED, "数据长度过长");
            }
            DataError::Nice => {
                let hex_string = ascii_to_hex(&input_data);
                data.set_output_data(hex_string.clone());
                ui.add(Label::new(RichText::new("十六进制:").color(Color32::BLUE)));
                ui.monospace(&hex_string);
                ui.separator();

                // 显示字节数统计
                let byte_count = input_data.len();
                ui.add(Label::new(
                    RichText::new("字节数:").color(Color32::GRAY),
                ));
                ui.monospace(format!("{} bytes", byte_count));

                // 显示格式化的十六进制（带空格分隔）
                ui.add(Label::new(
                    RichText::new("格式化:").color(Color32::GRAY),
                ));
                let formatted_hex = format_hex_with_spaces(&hex_string);
                ui.monospace(&formatted_hex);
            }
            DataError::FormatError => {
                ui.colored_label(Color32::RED, "格式错误");
            }
        };
    });
}

fn ascii_to_hex(ascii_string: &str) -> String {
    ascii_string
        .bytes()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join("")
}

fn format_hex_with_spaces(hex_string: &str) -> String {
    hex_string
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && i % 2 == 0 {
                format!(" {}", c)
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_to_hex_basic() {
        // 测试基本的ASCII字符
        let result = ascii_to_hex("Hello");
        assert_eq!(result, "48656C6C6F");
    }

    #[test]
    fn test_ascii_to_hex_world() {
        // 测试 "Hello World"
        let result = ascii_to_hex("Hello World");
        assert_eq!(result, "48656C6C6F20576F726C64");
    }

    #[test]
    fn test_ascii_to_hex_numbers() {
        // 测试数字字符
        let result = ascii_to_hex("123456");
        assert_eq!(result, "313233343536");
    }

    #[test]
    fn test_ascii_to_hex_empty() {
        // 测试空字符串
        let result = ascii_to_hex("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_ascii_to_hex_special_chars() {
        // 测试特殊字符
        let result = ascii_to_hex("!\"#$%&'");
        assert_eq!(result, "21222324252627");
    }

    #[test]
    fn test_ascii_to_hex_with_newline() {
        // 测试包含换行符
        let result = ascii_to_hex("Hello\nWorld");
        assert_eq!(result, "48656C6C6F0A576F726C64");
    }

    #[test]
    fn test_ascii_to_hex_with_tab() {
        // 测试包含制表符
        let result = ascii_to_hex("Hello\tWorld");
        assert_eq!(result, "48656C6C6F09576F726C64");
    }

    #[test]
    fn test_format_hex_with_spaces() {
        // 测试格式化十六进制字符串
        let result = format_hex_with_spaces("48656C6C6F");
        assert_eq!(result, "48 65 6C 6C 6F");
    }

    #[test]
    fn test_format_hex_with_spaces_empty() {
        // 测试空字符串格式化
        let result = format_hex_with_spaces("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_hex_with_spaces_single_byte() {
        // 测试单字节格式化
        let result = format_hex_with_spaces("48");
        assert_eq!(result, "48");
    }

    #[test]
    fn test_ascii_to_hex_chinese_utf8() {
        // 测试UTF-8编码的中文字符（每个中文字符通常占3个字节）
        let result = ascii_to_hex("你");
        // "你" 的UTF-8编码是 E4 BD A0
        assert_eq!(result, "E4BDA0");
    }

    #[test]
    fn test_ascii_to_hex_mixed_content() {
        // 测试混合内容
        let result = ascii_to_hex("A1!@");
        assert_eq!(result, "41312140");
    }

    #[test]
    fn test_ascii_to_hex_debug() {
        // 详细调试测试
        println!("ASCII转Hex测试开始...");

        let test_cases = vec![
            ("A", "41"),
            ("Hello", "48656C6C6F"),
            ("Hello World", "48656C6C6F20576F726C64"),
            ("123", "313233"),
            ("\n", "0A"),
            ("\t", "09"),
            (" ", "20"),
            ("!@#", "214023"),
        ];

        for (ascii_input, expected) in test_cases {
            println!("测试输入: {:?} -> 期望: {}", ascii_input, expected);
            let result = ascii_to_hex(ascii_input);
            println!("实际结果: {}", result);
            if result != expected {
                println!("❌ 不匹配! 期望: {}, 实际: {}", expected, result);
            } else {
                println!("✅ 匹配!");
            }
            println!("---");
        }
    }
}