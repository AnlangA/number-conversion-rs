use crate::core::errors::{ConversionError, ConversionResult, validate_not_empty, validate_length, validate_radix_chars};
use crate::core::models::ConversionData;

/// 浮点数转换器
pub struct FloatConverter;

impl FloatConverter {
    /// f32浮点数转换为十六进制编码
    pub fn f32_to_hex(data: &mut ConversionData) -> ConversionResult<()> {
        let input = data.cleaned_input();
        
        // 验证输入
        validate_not_empty(input)?;

        // 解析为f32
        let float_value = input.parse::<f32>()
            .map_err(|e| ConversionError::ParseError(format!("无法解析为f32: {}", e)))?;
        
        // 转换为十六进制编码
        let bits = float_value.to_bits();
        let hex_result = format!("{:08X}", bits);
        
        data.set_output(hex_result);
        Ok(())
    }

    /// 十六进制编码转换为f32浮点数
    pub fn hex_to_f32(data: &mut ConversionData) -> ConversionResult<()> {
        let input = data.cleaned_input();
        
        // 验证输入
        validate_not_empty(input)?;
        validate_length(input, 8)?;
        validate_radix_chars(input, 16)?;

        // 转换为u32然后转换为f32
        let bits = u32::from_str_radix(input, 16)
            .map_err(|e| ConversionError::ParseError(e.to_string()))?;
        
        let float_value = f32::from_bits(bits);
        
        // 检查是否为特殊值
        let result = if float_value.is_nan() {
            "NaN (Not a Number)".to_string()
        } else if float_value.is_infinite() {
            if float_value.is_sign_positive() {
                "+∞ (Positive Infinity)".to_string()
            } else {
                "-∞ (Negative Infinity)".to_string()
            }
        } else {
            float_value.to_string()
        };
        
        data.set_output(result);
        Ok(())
    }

    /// 分析f32的IEEE 754结构
    pub fn analyze_f32_structure(data: &mut ConversionData) -> ConversionResult<String> {
        let input = data.cleaned_input();
        
        // 验证输入
        validate_not_empty(input)?;
        validate_length(input, 8)?;
        validate_radix_chars(input, 16)?;

        // 转换为u32
        let bits = u32::from_str_radix(input, 16)
            .map_err(|e| ConversionError::ParseError(e.to_string()))?;
        
        // 提取IEEE 754各部分
        let sign = (bits >> 31) & 1;
        let exponent = (bits >> 23) & 0xFF;
        let mantissa = bits & 0x7FFFFF;
        
        // 计算实际值
        let float_value = f32::from_bits(bits);
        
        let analysis = format!(
            "IEEE 754 单精度浮点数分析:\n\
            原始十六进制: 0x{:08X}\n\
            二进制: {:032b}\n\
            符号位 (1位): {} ({})\n\
            指数位 (8位): {:08b} ({})\n\
            尾数位 (23位): {:023b} (0x{:06X})\n\
            浮点值: {}",
            bits,
            bits,
            sign,
            if sign == 0 { "正数" } else { "负数" },
            exponent,
            exponent,
            mantissa,
            mantissa,
            float_value
        );
        
        Ok(analysis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_to_hex() {
        let mut data = ConversionData::new();
        data.set_input("1.0".to_string());
        
        FloatConverter::f32_to_hex(&mut data).unwrap();
        assert_eq!(data.output(), "3F800000");
    }

    #[test]
    fn test_hex_to_f32() {
        let mut data = ConversionData::new();
        data.set_input("3F800000".to_string());
        
        FloatConverter::hex_to_f32(&mut data).unwrap();
        assert_eq!(data.output(), "1");
    }

    #[test]
    fn test_analyze_f32_structure() {
        let mut data = ConversionData::new();
        data.set_input("3F800000".to_string());
        
        let analysis = FloatConverter::analyze_f32_structure(&mut data).unwrap();
        assert!(analysis.contains("IEEE 754"));
        assert!(analysis.contains("浮点值: 1"));
    }
}
