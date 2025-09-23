use crate::core::errors::{
    validate_length, validate_not_empty, validate_radix_chars, ConversionError, ConversionResult,
};
use crate::core::models::ConversionData;
use num::BigUint;

/// 进制转换器
pub struct BaseConverter;

impl BaseConverter {}

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

        data.set_output(format!("16进制: {}\n10进制: {}", hex_result, dec_result));
        Ok(())
    }

    /// 十进制转换为其他进制
    pub fn from_decimal(data: &mut ConversionData) -> ConversionResult<()> {
        let input = data.cleaned_input();

        // 验证输入
        validate_not_empty(input)?;
        validate_radix_chars(input, 10)?;

        // 转换
        let number = input
            .parse::<u64>()
            .map_err(|e| ConversionError::ParseError(e.to_string()))?;

        let bin_result = BigUint::from(number).to_str_radix(2);
        let hex_result = BigUint::from(number).to_str_radix(16).to_uppercase();

        data.set_output(format!("2进制: {}\n16进制: {}", bin_result, hex_result));
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

        data.set_output(format!("2进制: {}\n10进制: {}", bin_result, dec_result));
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
