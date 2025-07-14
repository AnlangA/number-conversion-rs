use crate::data::*;
use eframe::egui;
use egui::*;

pub fn hex_ascii(data: &mut Data, ui: &mut Ui) {
    data.set_data_error(DataError::Nice);
    let mut input_data = String::new();

    ui.horizontal(|ui| {
        ui.label(RichText::from("Hex转ASCII").color(Color32::BLUE))
            .on_hover_text("输入十六进制字符串，自动转换为ASCII文本");
        let text_edit = TextEdit::singleline(&mut data.input_data).desired_width(400.0);
        ui.add(text_edit);

        // 移除空格和下划线做视觉分割
        let raw_data = data
            .ref_input_data()
            .clone()
            .replace(" ", "")
            .replace("_", "");

        if raw_data.is_empty() {
            data.set_data_error(DataError::LenNull);
        } else if raw_data.len() % 2 != 0 {
            data.set_data_error(DataError::FormatError);
        } else {
            // 验证是否为有效的十六进制字符
            input_data = raw_data
                .chars()
                .filter(|c| {
                    if !c.is_ascii_hexdigit() {
                        data.set_data_error(DataError::FormatError);
                        false
                    } else {
                        true
                    }
                })
                .collect();
        }
    });

    ui.horizontal(|ui| {
        match data.get_data_error() {
            DataError::FormatError => {
                ui.colored_label(Color32::RED, "请输入有效的十六进制字符（长度必须为偶数）")
            }
            DataError::LenNull => ui.colored_label(Color32::RED, "请输入十六进制数据"),
            DataError::LenOver => ui.colored_label(Color32::RED, "数据长度过长"),
            DataError::Nice => {
                match hex_to_ascii(&input_data) {
                    Ok(ascii_text) => {
                        data.set_output_data(ascii_text.clone());
                        ui.add(Label::new(RichText::new("ASCII文本:").color(Color32::BLUE)));
                        ui.monospace(&ascii_text);
                        ui.separator();

                        // 显示可打印字符统计
                        let printable_count = ascii_text
                            .chars()
                            .filter(|c| c.is_ascii_graphic() || *c == ' ')
                            .count();
                        ui.add(Label::new(
                            RichText::new("可打印字符数:").color(Color32::GRAY),
                        ));
                        ui.monospace(format!("{}/{}", printable_count, ascii_text.len()))
                    }
                    Err(err) => ui.colored_label(Color32::RED, format!("转换错误: {}", err)),
                }
            }
        };
    });
}

fn hex_to_ascii(hex_string: &str) -> Result<String, String> {
    if hex_string.len() % 2 != 0 {
        return Err("十六进制字符串长度必须为偶数".to_string());
    }

    let mut result = String::new();

    for i in (0..hex_string.len()).step_by(2) {
        let hex_pair = &hex_string[i..i + 2];
        match u8::from_str_radix(hex_pair, 16) {
            Ok(byte_value) => {
                // 将字节转换为字符，如果不是可打印字符则显示为替代字符
                if byte_value.is_ascii() {
                    let ch = byte_value as char;
                    if ch.is_ascii_control() && ch != '\n' && ch != '\t' && ch != '\r' {
                        result.push('.'); // 用点号表示控制字符
                    } else {
                        result.push(ch);
                    }
                } else {
                    result.push('?'); // 用问号表示非ASCII字符
                }
            }
            Err(_) => {
                return Err(format!("无效的十六进制值: {}", hex_pair));
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_ascii_basic() {
        // 测试基本的ASCII字符
        let result = hex_to_ascii("48656C6C6F").unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_hex_to_ascii_world() {
        // 测试 "Hello World"
        let result = hex_to_ascii("48656C6C6F20576F726C64").unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_hex_to_ascii_numbers() {
        // 测试数字字符
        let result = hex_to_ascii("313233343536").unwrap();
        assert_eq!(result, "123456");
    }

    #[test]
    fn test_hex_to_ascii_mixed_case() {
        // 测试大小写混合
        let result = hex_to_ascii("48656c6c6f").unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_hex_to_ascii_with_newline() {
        // 测试包含换行符
        let result = hex_to_ascii("48656C6C6F0A576F726C64").unwrap();
        assert_eq!(result, "Hello\nWorld");
    }

    #[test]
    fn test_hex_to_ascii_with_tab() {
        // 测试包含制表符
        let result = hex_to_ascii("48656C6C6F09576F726C64").unwrap();
        assert_eq!(result, "Hello\tWorld");
    }

    #[test]
    fn test_hex_to_ascii_control_chars() {
        // 测试控制字符会被替换为点号
        let result = hex_to_ascii("48656C6C6F01576F726C64").unwrap();
        assert_eq!(result, "Hello.World");
    }

    #[test]
    fn test_hex_to_ascii_non_ascii() {
        // 测试非ASCII字符会被替换为问号
        let result = hex_to_ascii("48656C6C6FFF576F726C64").unwrap();
        assert_eq!(result, "Hello?World");
    }

    #[test]
    fn test_hex_to_ascii_empty() {
        // 测试空字符串
        let result = hex_to_ascii("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_hex_to_ascii_odd_length() {
        // 测试奇数长度应该返回错误
        let result = hex_to_ascii("48656C6C6");
        assert!(result.is_err());
    }

    #[test]
    fn test_hex_to_ascii_invalid_hex() {
        // 测试无效的十六进制字符
        let result = hex_to_ascii("48656C6C6G");
        assert!(result.is_err());
    }

    #[test]
    fn test_hex_to_ascii_special_chars() {
        // 测试特殊可打印字符
        let result = hex_to_ascii("21222324252627").unwrap(); // !"#$%&'
        assert_eq!(result, "!\"#$%&'");
    }

    #[test]
    fn test_hex_to_ascii_debug() {
        // 详细调试测试
        println!("测试开始...");

        // 测试简单的"A"
        let test_cases = vec![
            ("41", "A"),
            ("48656C6C6F", "Hello"),
            ("48656C6C6F20576F726C64", "Hello World"),
            ("313233", "123"),
            ("0A", "\n"),
            ("09", "\t"),
            ("20", " "),
            ("7F", "?"), // DEL字符应该被替换
            ("80", "?"), // 非ASCII字符
        ];

        for (hex_input, expected) in test_cases {
            println!("测试输入: {} -> 期望: {:?}", hex_input, expected);
            match hex_to_ascii(hex_input) {
                Ok(result) => {
                    println!("实际结果: {:?}", result);
                    if result != expected {
                        println!("❌ 不匹配! 期望: {:?}, 实际: {:?}", expected, result);
                    } else {
                        println!("✅ 匹配!");
                    }
                }
                Err(e) => {
                    println!("❌ 错误: {}", e);
                }
            }
            println!("---");
        }
    }
}
