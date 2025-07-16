//! 验证工具函数

use crate::core::errors::{ConversionError, ConversionResult};

/// 验证工具
pub struct Validator;

impl Validator {
    /// 验证字符串是否为有效的进制数字
    /// 
    /// # 参数
    /// * `input` - 输入字符串
    /// * `radix` - 进制（2, 8, 10, 16等）
    /// 
    /// # 示例
    /// ```
    /// use number_conversion::utils::validation::Validator;
    /// 
    /// assert!(Validator::is_valid_radix("1010", 2).is_ok());
    /// assert!(Validator::is_valid_radix("FF", 16).is_ok());
    /// assert!(Validator::is_valid_radix("G", 16).is_err());
    /// ```
    pub fn is_valid_radix(input: &str, radix: u32) -> ConversionResult<()> {
        if input.is_empty() {
            return Err(ConversionError::EmptyInput);
        }

        for ch in input.chars() {
            if !ch.is_digit(radix) {
                let radix_name = Self::radix_name(radix);
                return Err(ConversionError::InvalidFormat {
                    expected: format!("{}字符", radix_name),
                    got: format!("字符 '{}'", ch),
                });
            }
        }
        Ok(())
    }

    /// 验证十六进制字符串
    /// 
    /// # 示例
    /// ```
    /// use number_conversion::utils::validation::Validator;
    /// 
    /// assert!(Validator::is_valid_hex("ABCD1234").is_ok());
    /// assert!(Validator::is_valid_hex("GHIJ").is_err());
    /// ```
    pub fn is_valid_hex(input: &str) -> ConversionResult<()> {
        Self::is_valid_radix(input, 16)
    }

    /// 验证二进制字符串
    /// 
    /// # 示例
    /// ```
    /// use number_conversion::utils::validation::Validator;
    /// 
    /// assert!(Validator::is_valid_binary("101010").is_ok());
    /// assert!(Validator::is_valid_binary("102").is_err());
    /// ```
    pub fn is_valid_binary(input: &str) -> ConversionResult<()> {
        Self::is_valid_radix(input, 2)
    }

    /// 验证十进制字符串
    /// 
    /// # 示例
    /// ```
    /// use number_conversion::utils::validation::Validator;
    /// 
    /// assert!(Validator::is_valid_decimal("12345").is_ok());
    /// assert!(Validator::is_valid_decimal("123A5").is_err());
    /// ```
    pub fn is_valid_decimal(input: &str) -> ConversionResult<()> {
        Self::is_valid_radix(input, 10)
    }

    /// 验证ASCII文本（只包含可打印字符）
    /// 
    /// # 示例
    /// ```
    /// use number_conversion::utils::validation::Validator;
    /// 
    /// assert!(Validator::is_valid_ascii_text("Hello World").is_ok());
    /// ```
    pub fn is_valid_ascii_text(input: &str) -> ConversionResult<()> {
        if input.is_empty() {
            return Err(ConversionError::EmptyInput);
        }

        for ch in input.chars() {
            if !ch.is_ascii() {
                return Err(ConversionError::InvalidFormat {
                    expected: "ASCII字符".to_string(),
                    got: format!("非ASCII字符 '{}'", ch),
                });
            }
        }
        Ok(())
    }

    /// 验证浮点数字符串
    /// 
    /// # 示例
    /// ```
    /// use number_conversion::utils::validation::Validator;
    /// 
    /// assert!(Validator::is_valid_float("3.14159").is_ok());
    /// assert!(Validator::is_valid_float("abc").is_err());
    /// ```
    pub fn is_valid_float(input: &str) -> ConversionResult<()> {
        if input.is_empty() {
            return Err(ConversionError::EmptyInput);
        }

        input.parse::<f32>().map_err(|_| ConversionError::InvalidFormat {
            expected: "浮点数".to_string(),
            got: input.to_string(),
        })?;

        Ok(())
    }

    /// 验证字符串长度
    /// 
    /// # 参数
    /// * `input` - 输入字符串
    /// * `min_length` - 最小长度
    /// * `max_length` - 最大长度
    /// 
    /// # 示例
    /// ```
    /// use number_conversion::utils::validation::Validator;
    /// 
    /// assert!(Validator::is_valid_length("Hello", 1, 10).is_ok());
    /// assert!(Validator::is_valid_length("", 1, 10).is_err());
    /// ```
    pub fn is_valid_length(input: &str, min_length: usize, max_length: usize) -> ConversionResult<()> {
        let len = input.len();
        
        if len < min_length {
            return Err(ConversionError::InputTooLong {
                max_length: min_length,
                actual: len,
            });
        }
        
        if len > max_length {
            return Err(ConversionError::InputTooLong {
                max_length,
                actual: len,
            });
        }
        
        Ok(())
    }

    /// 验证数值是否在指定范围内
    /// 
    /// # 示例
    /// ```
    /// use number_conversion::utils::validation::Validator;
    /// 
    /// assert!(Validator::is_in_range(50, 0, 100).is_ok());
    /// assert!(Validator::is_in_range(150, 0, 100).is_err());
    /// ```
    pub fn is_in_range<T>(value: T, min: T, max: T) -> ConversionResult<()>
    where
        T: PartialOrd + std::fmt::Display + Copy,
    {
        if value < min || value > max {
            return Err(ConversionError::ValueOutOfRange {
                min: min.to_string(),
                max: max.to_string(),
                value: value.to_string(),
            });
        }
        Ok(())
    }

    /// 获取进制名称
    fn radix_name(radix: u32) -> &'static str {
        match radix {
            2 => "二进制",
            8 => "八进制",
            10 => "十进制",
            16 => "十六进制",
            _ => "未知进制",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_hex() {
        assert!(Validator::is_valid_hex("ABCD1234").is_ok());
        assert!(Validator::is_valid_hex("abcd1234").is_ok());
        assert!(Validator::is_valid_hex("GHIJ").is_err());
        assert!(Validator::is_valid_hex("").is_err());
    }

    #[test]
    fn test_is_valid_binary() {
        assert!(Validator::is_valid_binary("101010").is_ok());
        assert!(Validator::is_valid_binary("102").is_err());
        assert!(Validator::is_valid_binary("").is_err());
    }

    #[test]
    fn test_is_valid_decimal() {
        assert!(Validator::is_valid_decimal("12345").is_ok());
        assert!(Validator::is_valid_decimal("123A5").is_err());
        assert!(Validator::is_valid_decimal("").is_err());
    }

    #[test]
    fn test_is_valid_ascii_text() {
        assert!(Validator::is_valid_ascii_text("Hello World").is_ok());
        assert!(Validator::is_valid_ascii_text("").is_err());
    }

    #[test]
    fn test_is_valid_float() {
        assert!(Validator::is_valid_float("3.14159").is_ok());
        assert!(Validator::is_valid_float("123").is_ok());
        assert!(Validator::is_valid_float("abc").is_err());
        assert!(Validator::is_valid_float("").is_err());
    }

    #[test]
    fn test_is_valid_length() {
        assert!(Validator::is_valid_length("Hello", 1, 10).is_ok());
        assert!(Validator::is_valid_length("", 1, 10).is_err());
        assert!(Validator::is_valid_length("Very long string", 1, 10).is_err());
    }

    #[test]
    fn test_is_in_range() {
        assert!(Validator::is_in_range(50, 0, 100).is_ok());
        assert!(Validator::is_in_range(150, 0, 100).is_err());
        assert!(Validator::is_in_range(-10, 0, 100).is_err());
    }
}
