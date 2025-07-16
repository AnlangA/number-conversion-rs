use crate::core::errors::{ConversionError, ConversionResult, validate_not_empty, validate_radix_chars};
use crate::core::models::ConversionData;

/// 文本转换器
pub struct TextConverter;

impl TextConverter {
    /// ASCII文本转换为十六进制
    pub fn ascii_to_hex(data: &mut ConversionData) -> ConversionResult<()> {
        let input = data.raw_input();
        
        // 验证输入
        validate_not_empty(input)?;

        // 转换每个字符为十六进制
        let hex_result: String = input
            .chars()
            .map(|c| format!("{:02X}", c as u8))
            .collect::<Vec<String>>()
            .join(" ");
        
        data.set_output(hex_result);
        Ok(())
    }

    /// 十六进制转换为ASCII文本
    pub fn hex_to_ascii(data: &mut ConversionData) -> ConversionResult<()> {
        let input = data.raw_input().trim();

        // 验证输入
        validate_not_empty(input)?;

        // 移除空格和下划线，并转换为大写
        let clean_hex = input
            .chars()
            .filter(|&c| c != ' ' && c != '_')
            .collect::<String>()
            .to_uppercase();

        validate_radix_chars(&clean_hex, 16)?;
        
        // 确保长度为偶数
        if clean_hex.len() % 2 != 0 {
            return Err(ConversionError::InvalidFormat {
                expected: "偶数长度的十六进制字符串".to_string(),
                got: format!("长度为 {} 的字符串", clean_hex.len()),
            });
        }

        // 转换为ASCII字符
        let mut ascii_result = String::new();
        for chunk in clean_hex.as_bytes().chunks(2) {
            let hex_str = std::str::from_utf8(chunk)
                .map_err(|e| ConversionError::ParseError(e.to_string()))?;
            
            let byte_value = u8::from_str_radix(hex_str, 16)
                .map_err(|e| ConversionError::ParseError(e.to_string()))?;
            
            // 检查是否为可打印ASCII字符
            if byte_value.is_ascii() && byte_value >= 32 && byte_value <= 126 {
                ascii_result.push(byte_value as char);
            } else {
                ascii_result.push_str(&format!("[0x{:02X}]", byte_value));
            }
        }
        
        data.set_output(ascii_result);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_to_hex() {
        let mut data = ConversionData::new();
        data.set_input("Hello".to_string());
        
        TextConverter::ascii_to_hex(&mut data).unwrap();
        assert_eq!(data.output(), "48 65 6C 6C 6F");
    }

    #[test]
    fn test_hex_to_ascii() {
        let mut data = ConversionData::new();
        data.set_input("48 65 6C 6C 6F".to_string());
        
        TextConverter::hex_to_ascii(&mut data).unwrap();
        assert_eq!(data.output(), "Hello");
    }

    #[test]
    fn test_hex_to_ascii_non_printable() {
        let mut data = ConversionData::new();
        data.set_input("00 48 65 6C 6C 6F 00".to_string());
        
        TextConverter::hex_to_ascii(&mut data).unwrap();
        assert_eq!(data.output(), "[0x00]Hello[0x00]");
    }
}
