use crate::core::errors::ConversionError;
use crate::core::validators::ValidationResult;

/// 转换器的输入输出数据
#[derive(Debug, Clone)]
pub struct ConversionData {
    /// 原始输入数据
    raw_input: String,
    /// 清理后的输入数据（移除下划线等分隔符）
    cleaned_input: String,
    /// 输出数据
    output: String,
    /// 分析结果数据
    analysis: Option<String>,
    /// 最后的错误状态
    last_error: Option<ConversionError>,
}

impl ConversionData {
    /// 创建新的转换数据实例
    pub fn new() -> Self {
        Self {
            raw_input: String::new(),
            cleaned_input: String::new(),
            output: String::new(),
            analysis: None,
            last_error: None,
        }
    }

    /// 设置输入数据
    pub fn set_input(&mut self, input: String) {
        self.raw_input = input;
        self.cleaned_input = self.clean_input(&self.raw_input);
        self.clear_error();
        self.clear_analysis();
    }

    /// 使用验证结果设置输入数据
    pub fn set_input_with_validation_result(&mut self, validation_result: ValidationResult) {
        self.raw_input = validation_result.display_input;
        self.cleaned_input = validation_result.cleaned_input;
        self.clear_analysis();

        if let Some(error) = validation_result.error {
            self.set_error(error);
        } else {
            self.clear_error();
        }
    }

    /// 设置输入数据并验证格式（用于特定进制）
    pub fn set_input_with_validation(&mut self, input: String, radix: u32) -> bool {
        // 先尝试清理输入
        let cleaned = self.clean_input(&input);

        // 验证每个字符是否符合指定进制
        let mut valid_chars = String::new();
        let mut has_invalid = false;

        for ch in cleaned.chars() {
            if ch.is_digit(radix) {
                valid_chars.push(ch);
            } else {
                has_invalid = true;
            }
        }

        // 更新输入为有效字符
        self.raw_input = if radix == 16 {
            // 十六进制保持大写
            self.format_for_display(&valid_chars)
        } else {
            self.format_for_display(&valid_chars)
        };

        self.cleaned_input = valid_chars;
        self.clear_analysis();

        // 如果有无效字符，设置错误
        if has_invalid {
            let radix_name = match radix {
                2 => "二进制",
                8 => "八进制",
                10 => "十进制",
                16 => "十六进制",
                _ => "未知进制",
            };
            self.set_error(ConversionError::InvalidFormat {
                expected: format!("{}字符", radix_name),
                got: "包含无效字符，已自动删除".to_string(),
            });
            false
        } else {
            self.clear_error();
            true
        }
    }

    /// 设置输入数据并验证浮点数格式
    pub fn set_input_with_float_validation(&mut self, input: String) -> bool {
        // 验证浮点数字符（数字、小数点、负号）
        let mut valid_chars = String::new();
        let mut has_invalid = false;
        let mut has_dot = false;
        let mut has_minus = false;

        for (i, ch) in input.chars().enumerate() {
            match ch {
                '0'..='9' => valid_chars.push(ch),
                '.' if !has_dot => {
                    has_dot = true;
                    valid_chars.push(ch);
                },
                '-' if i == 0 && !has_minus => {
                    has_minus = true;
                    valid_chars.push(ch);
                },
                ' ' | '_' => {}, // 忽略分隔符
                _ => has_invalid = true,
            }
        }

        self.raw_input = valid_chars.clone();
        self.cleaned_input = valid_chars;
        self.clear_analysis();

        // 如果有无效字符，设置错误
        if has_invalid {
            self.set_error(ConversionError::InvalidFormat {
                expected: "浮点数字符".to_string(),
                got: "包含无效字符，已自动删除".to_string(),
            });
            false
        } else {
            self.clear_error();
            true
        }
    }

    /// 设置输入数据并验证十六进制格式（用于文本转换，支持空格分隔符）
    pub fn set_input_with_hex_text_validation(&mut self, input: String) -> bool {
        // 验证十六进制字符和空格
        let mut valid_chars = String::new();
        let mut has_invalid = false;

        for ch in input.chars() {
            match ch {
                '0'..='9' | 'A'..='F' | 'a'..='f' | ' ' | '_' => {
                    if ch.is_ascii_hexdigit() {
                        valid_chars.push(ch.to_ascii_uppercase());
                    } else if ch == ' ' {
                        valid_chars.push(' ');
                    }
                    // 忽略下划线
                },
                _ => has_invalid = true,
            }
        }

        self.raw_input = valid_chars.clone();
        self.cleaned_input = valid_chars.replace(" ", "").replace("_", "");
        self.clear_analysis();

        // 如果有无效字符，设置错误
        if has_invalid {
            self.set_error(ConversionError::InvalidFormat {
                expected: "十六进制字符和空格".to_string(),
                got: "包含无效字符，已自动删除".to_string(),
            });
            false
        } else {
            self.clear_error();
            true
        }
    }

    /// 获取原始输入数据的引用
    pub fn raw_input(&self) -> &str {
        &self.raw_input
    }

    /// 获取原始输入数据的可变引用（用于UI绑定）
    pub fn raw_input_mut(&mut self) -> &mut String {
        &mut self.raw_input
    }

    /// 获取清理后的输入数据
    pub fn cleaned_input(&self) -> &str {
        &self.cleaned_input
    }

    /// 设置输出数据
    pub fn set_output(&mut self, output: String) {
        self.output = output;
    }

    /// 获取输出数据
    pub fn output(&self) -> &str {
        &self.output
    }

    /// 设置分析结果
    pub fn set_analysis(&mut self, analysis: String) {
        self.analysis = Some(analysis);
    }

    /// 获取分析结果
    pub fn analysis(&self) -> Option<&str> {
        self.analysis.as_deref()
    }

    /// 清除分析结果
    pub fn clear_analysis(&mut self) {
        self.analysis = None;
    }

    /// 设置错误
    pub fn set_error(&mut self, error: ConversionError) {
        self.last_error = Some(error);
    }

    /// 清除错误
    pub fn clear_error(&mut self) {
        self.last_error = None;
    }

    /// 获取最后的错误
    pub fn last_error(&self) -> Option<&ConversionError> {
        self.last_error.as_ref()
    }

    /// 检查是否有错误
    pub fn has_error(&self) -> bool {
        self.last_error.is_some()
    }

    /// 更新清理后的输入（当原始输入改变时调用）
    pub fn update_cleaned_input(&mut self) {
        self.cleaned_input = self.clean_input(&self.raw_input);
        self.clear_analysis();
    }

    /// 清理输入数据，移除下划线和空格
    fn clean_input(&self, input: &str) -> String {
        input
            .chars()
            .filter(|&c| c != '_' && c != ' ')
            .collect::<String>()
            .to_uppercase()
    }

    /// 格式化输出数据，添加下划线分隔符以提高可读性
    pub fn format_output_with_separator(&self) -> String {
        self.format_with_separator(&self.output)
    }

    /// 为字符串添加下划线分隔符
    fn format_with_separator(&self, data: &str) -> String {
        if data.contains('.') {
            // 处理浮点数
            let parts: Vec<&str> = data.split('.').collect();
            if parts.len() == 2 {
                let before_dot = self.add_underscores_reverse(parts[0]);
                let after_dot = parts[1];
                format!("{}.{}", before_dot, after_dot)
            } else {
                data.to_string()
            }
        } else {
            // 处理整数
            self.add_underscores_reverse(data)
        }
    }

    /// 格式化字符串用于显示（添加分隔符）
    fn format_for_display(&self, data: &str) -> String {
        if data.len() > 4 {
            self.format_with_separator(data)
        } else {
            data.to_string()
        }
    }

    /// 从右到左每4位添加下划线
    fn add_underscores_reverse(&self, data: &str) -> String {
        let reversed: String = data.chars().rev().collect();
        let mut result = String::new();
        
        for (i, c) in reversed.chars().enumerate() {
            if i > 0 && i % 4 == 0 {
                result.push('_');
            }
            result.push(c);
        }
        
        result.chars().rev().collect()
    }
}

impl Default for ConversionData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_input() {
        let mut data = ConversionData::new();
        data.set_input("A1_B2 C3".to_string());
        assert_eq!(data.cleaned_input(), "A1B2C3");
    }

    #[test]
    fn test_format_with_separator() {
        let data = ConversionData::new();
        assert_eq!(data.format_with_separator("12345678"), "1234_5678");
        assert_eq!(data.format_with_separator("123.456"), "123.456");
    }
}
