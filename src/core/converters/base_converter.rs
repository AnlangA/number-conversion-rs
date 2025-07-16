use crate::core::errors::{ConversionError, ConversionResult, validate_not_empty, validate_length, validate_radix_chars};
use crate::core::models::ConversionData;
use num::BigUint;

/// 进制转换器
pub struct BaseConverter;

impl BaseConverter {
    /// 格式化二进制输出（4位分组）
    fn format_binary_output(binary: &str) -> String {
        if binary.len() > 4 {
            Self::add_separators(binary, '_', 4)
        } else {
            binary.to_string()
        }
    }

    /// 格式化十进制输出（4位分组）
    fn format_decimal_output(decimal: &str) -> String {
        if decimal.len() > 4 {
            Self::add_separators(decimal, '_', 4)
        } else {
            decimal.to_string()
        }
    }

    /// 格式化十六进制输出（4位分组）
    fn format_hex_output(hex: &str) -> String {
        if hex.len() > 4 {
            Self::add_separators(hex, '_', 4)
        } else {
            hex.to_string()
        }
    }

    /// 添加分隔符
    fn add_separators(input: &str, separator: char, group_size: usize) -> String {
        let reversed: String = input.chars().rev().collect();
        let mut result = String::new();

        for (i, c) in reversed.chars().enumerate() {
            if i > 0 && i % group_size == 0 {
                result.push(separator);
            }
            result.push(c);
        }

        result.chars().rev().collect()
    }
}

impl BaseConverter {
    /// 二进制转换为其他进制
    pub fn from_binary(data: &mut ConversionData) -> ConversionResult<()> {
        let input = data.cleaned_input();
        
        // 验证输入
        validate_not_empty(input)?;
        validate_length(input, 64)?;
        validate_radix_chars(input, 2)?;

        // 转换
        let number = u64::from_str_radix(input, 2)
            .map_err(|e| ConversionError::ParseError(e.to_string()))?;
        
        let hex_result = BigUint::from(number).to_str_radix(16).to_uppercase();
        let dec_result = BigUint::from(number).to_str_radix(10);

        let formatted_hex = Self::format_hex_output(&hex_result);
        let formatted_dec = Self::format_decimal_output(&dec_result);

        data.set_output(format!("16进制: {}\n10进制: {}", formatted_hex, formatted_dec));
        Ok(())
    }

    /// 十进制转换为其他进制
    pub fn from_decimal(data: &mut ConversionData) -> ConversionResult<()> {
        let input = data.cleaned_input();
        
        // 验证输入
        validate_not_empty(input)?;
        validate_radix_chars(input, 10)?;

        // 转换
        let number = input.parse::<u64>()
            .map_err(|e| ConversionError::ParseError(e.to_string()))?;
        
        let bin_result = BigUint::from(number).to_str_radix(2);
        let hex_result = BigUint::from(number).to_str_radix(16).to_uppercase();

        let formatted_bin = Self::format_binary_output(&bin_result);
        let formatted_hex = Self::format_hex_output(&hex_result);

        data.set_output(format!("2进制: {}\n16进制: {}", formatted_bin, formatted_hex));
        Ok(())
    }

    /// 十六进制转换为其他进制
    pub fn from_hexadecimal(data: &mut ConversionData) -> ConversionResult<()> {
        let input = data.cleaned_input();
        
        // 验证输入
        validate_not_empty(input)?;
        validate_length(input, 16)?;
        validate_radix_chars(input, 16)?;

        // 转换
        let number = u64::from_str_radix(input, 16)
            .map_err(|e| ConversionError::ParseError(e.to_string()))?;
        
        let bin_result = BigUint::from(number).to_str_radix(2);
        let dec_result = BigUint::from(number).to_str_radix(10);

        let formatted_bin = Self::format_binary_output(&bin_result);
        let formatted_dec = Self::format_decimal_output(&dec_result);

        data.set_output(format!("2进制: {}\n10进制: {}", formatted_bin, formatted_dec));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_conversion() {
        let mut data = ConversionData::new();
        data.set_input("1010".to_string());
        
        BaseConverter::from_binary(&mut data).unwrap();
        assert!(data.output().contains("16进制: A"));
        assert!(data.output().contains("10进制: 10"));
    }

    #[test]
    fn test_decimal_conversion() {
        let mut data = ConversionData::new();
        data.set_input("10".to_string());
        
        BaseConverter::from_decimal(&mut data).unwrap();
        assert!(data.output().contains("2进制: 1010"));
        assert!(data.output().contains("16进制: A"));
    }

    #[test]
    fn test_hex_conversion() {
        let mut data = ConversionData::new();
        data.set_input("A".to_string());
        
        BaseConverter::from_hexadecimal(&mut data).unwrap();
        assert!(data.output().contains("2进制: 1010"));
        assert!(data.output().contains("10进制: 10"));
    }
}
