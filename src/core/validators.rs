use crate::core::errors::ConversionError;

/// 输入验证结果
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// 清理后的有效输入
    pub cleaned_input: String,
    /// 格式化后的显示输入
    pub display_input: String,
    /// 是否有无效字符被移除
    pub has_invalid_chars: bool,
    /// 错误信息（如果有）
    pub error: Option<ConversionError>,
}

impl ValidationResult {
    /// 创建成功的验证结果
    pub fn success(cleaned: String, display: String) -> Self {
        Self {
            cleaned_input: cleaned,
            display_input: display,
            has_invalid_chars: false,
            error: None,
        }
    }

    /// 创建带警告的验证结果（有无效字符被移除）
    pub fn warning(cleaned: String, display: String, error: ConversionError) -> Self {
        Self {
            cleaned_input: cleaned,
            display_input: display,
            has_invalid_chars: true,
            error: Some(error),
        }
    }

    /// 创建失败的验证结果
    pub fn failure(error: ConversionError) -> Self {
        Self {
            cleaned_input: String::new(),
            display_input: String::new(),
            has_invalid_chars: true,
            error: Some(error),
        }
    }

    /// 是否验证成功（没有错误）
    pub fn is_valid(&self) -> bool {
        self.error.is_none()
    }
}

/// 二进制输入验证器
pub struct BinaryValidator;

impl BinaryValidator {
    /// 验证二进制输入
    pub fn validate(input: &str) -> ValidationResult {
        if input.trim().is_empty() {
            return ValidationResult::success(String::new(), String::new());
        }

        let mut valid_chars = String::new();
        let mut display_chars = String::new();
        let mut has_invalid = false;

        for ch in input.chars() {
            match ch {
                '0' | '1' => {
                    valid_chars.push(ch);
                    display_chars.push(ch);
                }
                ' ' | '_' | ',' => {
                    display_chars.push(ch); // 保留分隔符用于显示
                }
                _ => has_invalid = true,
            }
        }

        let display = Self::format_for_display(&display_chars);

        if has_invalid {
            ValidationResult::warning(
                valid_chars,
                display,
                ConversionError::InvalidFormat {
                    expected: "二进制字符(0,1)".to_string(),
                    got: "包含无效字符，已自动删除".to_string(),
                },
            )
        } else {
            ValidationResult::success(valid_chars, display)
        }
    }

    fn format_for_display(input: &str) -> String {
        input.to_string()
    }
}

/// 十进制输入验证器
pub struct DecimalValidator;

impl DecimalValidator {
    /// 验证十进制输入
    pub fn validate(input: &str) -> ValidationResult {
        if input.trim().is_empty() {
            return ValidationResult::success(String::new(), String::new());
        }

        let mut valid_chars = String::new();
        let mut display_chars = String::new();
        let mut has_invalid = false;

        for ch in input.chars() {
            match ch {
                '0'..='9' => {
                    valid_chars.push(ch);
                    display_chars.push(ch);
                }
                ' ' | '_' | ',' => {
                    display_chars.push(ch); // 保留分隔符用于显示
                }
                _ => has_invalid = true,
            }
        }

        let display = Self::format_for_display(&display_chars);

        if has_invalid {
            ValidationResult::warning(
                valid_chars,
                display,
                ConversionError::InvalidFormat {
                    expected: "十进制字符(0-9)".to_string(),
                    got: "包含无效字符，已自动删除".to_string(),
                },
            )
        } else {
            ValidationResult::success(valid_chars, display)
        }
    }

    fn format_for_display(input: &str) -> String {
        input.to_string()
    }
}

/// 十六进制输入验证器
pub struct HexValidator;

impl HexValidator {
    /// 验证十六进制输入
    pub fn validate(input: &str) -> ValidationResult {
        if input.trim().is_empty() {
            return ValidationResult::success(String::new(), String::new());
        }

        let mut valid_chars = String::new();
        let mut display_chars = String::new();
        let mut has_invalid = false;

        for ch in input.chars() {
            match ch {
                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                    valid_chars.push(ch.to_ascii_uppercase());
                    display_chars.push(ch.to_ascii_uppercase());
                }
                ' ' | '_' | ',' => {
                    display_chars.push(ch); // 保留分隔符用于显示
                }
                _ => has_invalid = true,
            }
        }

        let display = Self::format_for_display(&display_chars);

        if has_invalid {
            ValidationResult::warning(
                valid_chars,
                display,
                ConversionError::InvalidFormat {
                    expected: "十六进制字符(0-9,A-F)".to_string(),
                    got: "包含无效字符，已自动删除".to_string(),
                },
            )
        } else {
            ValidationResult::success(valid_chars, display)
        }
    }

    fn format_for_display(input: &str) -> String {
        input.to_string()
    }
}

/// 浮点数输入验证器
pub struct FloatValidator;

impl FloatValidator {
    /// 验证浮点数输入
    pub fn validate(input: &str) -> ValidationResult {
        if input.trim().is_empty() {
            return ValidationResult::success(String::new(), String::new());
        }

        let mut valid_chars = String::new();
        let mut display_chars = String::new();
        let mut has_invalid = false;
        let mut has_dot = false;
        let mut has_minus = false;

        for (i, ch) in input.chars().enumerate() {
            match ch {
                '0'..='9' => {
                    valid_chars.push(ch);
                    display_chars.push(ch);
                }
                '.' if !has_dot => {
                    has_dot = true;
                    valid_chars.push(ch);
                    display_chars.push(ch);
                }
                '-' if i == 0 && !has_minus => {
                    has_minus = true;
                    valid_chars.push(ch);
                    display_chars.push(ch);
                }
                ' ' | '_' | ',' => {
                    display_chars.push(ch); // 保留分隔符用于显示
                }
                _ => has_invalid = true,
            }
        }

        if has_invalid {
            ValidationResult::warning(
                valid_chars.clone(),
                display_chars,
                ConversionError::InvalidFormat {
                    expected: "浮点数字符(数字,小数点,负号)".to_string(),
                    got: "包含无效字符，已自动删除".to_string(),
                },
            )
        } else {
            ValidationResult::success(valid_chars.clone(), display_chars)
        }
    }
}

/// 十六进制文本输入验证器（用于十六进制转ASCII，支持空格分隔）
pub struct HexTextValidator;

impl HexTextValidator {
    /// 验证十六进制文本输入（支持空格分隔）
    pub fn validate(input: &str) -> ValidationResult {
        if input.trim().is_empty() {
            return ValidationResult::success(String::new(), String::new());
        }

        let mut valid_chars = String::new();
        let mut has_invalid = false;

        for ch in input.chars() {
            match ch {
                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                    valid_chars.push(ch.to_ascii_uppercase());
                }
                ' ' => valid_chars.push(' '), // 保留空格作为分隔符
                '_' | ',' => {}               // 忽略下划线和逗号
                _ => has_invalid = true,
            }
        }

        // 清理多余的空格
        let display = Self::normalize_spaces(&valid_chars);

        if has_invalid {
            ValidationResult::warning(
                display.clone(),
                display,
                ConversionError::InvalidFormat {
                    expected: "十六进制字符和空格".to_string(),
                    got: "包含无效字符，已自动删除".to_string(),
                },
            )
        } else {
            ValidationResult::success(display.clone(), display)
        }
    }

    fn normalize_spaces(input: &str) -> String {
        // 移除多余的空格，确保十六进制字符之间有适当的分隔
        input.split_whitespace().collect::<Vec<&str>>().join(" ")
    }
}

/// ASCII文本输入验证器
pub struct AsciiValidator;

impl AsciiValidator {
    /// 验证ASCII文本输入
    pub fn validate(input: &str) -> ValidationResult {
        if input.is_empty() {
            return ValidationResult::success(String::new(), String::new());
        }

        let mut valid_chars = String::new();
        let mut has_invalid = false;

        for ch in input.chars() {
            if ch.is_ascii() {
                valid_chars.push(ch);
            } else {
                has_invalid = true;
            }
        }

        if has_invalid {
            ValidationResult::warning(
                valid_chars.clone(),
                valid_chars,
                ConversionError::InvalidFormat {
                    expected: "ASCII字符".to_string(),
                    got: "包含非ASCII字符，已自动删除".to_string(),
                },
            )
        } else {
            ValidationResult::success(valid_chars.clone(), valid_chars)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_validator() {
        let result = BinaryValidator::validate("1012");
        assert!(result.has_invalid_chars);
        assert_eq!(result.cleaned_input, "101");
    }

    #[test]
    fn test_hex_validator() {
        let result = HexValidator::validate("A1G2");
        assert!(result.has_invalid_chars);
        assert_eq!(result.cleaned_input, "A12");
    }

    #[test]
    fn test_hex_text_validator() {
        let result = HexTextValidator::validate("48 65 6C 6C 6F");
        assert!(!result.has_invalid_chars);
        assert_eq!(result.cleaned_input, "48 65 6C 6C 6F");
    }
}
