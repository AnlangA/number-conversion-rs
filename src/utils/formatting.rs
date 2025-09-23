//! 格式化工具函数

/// 格式化工具
pub struct Formatter;

impl Formatter {
    /// 为数字字符串添加分隔符以提高可读性
    ///
    /// # 参数
    /// * `input` - 输入字符串
    /// * `separator` - 分隔符字符
    /// * `group_size` - 分组大小
    ///
    /// # 示例
    /// ```
    /// use number_conversion::utils::formatting::Formatter;
    ///
    /// let result = Formatter::add_separator("12345678", '_', 4);
    /// assert_eq!(result, "1234_5678");
    /// ```
    pub fn add_separator(input: &str, separator: char, group_size: usize) -> String {
        if input.is_empty() || group_size == 0 {
            return input.to_string();
        }

        // 处理包含小数点的情况
        if let Some(dot_pos) = input.find('.') {
            let (before_dot, after_dot) = input.split_at(dot_pos);
            let formatted_before =
                Self::add_separator_to_integer(before_dot, separator, group_size);
            format!("{}{}", formatted_before, after_dot)
        } else {
            Self::add_separator_to_integer(input, separator, group_size)
        }
    }

    /// 为整数字符串添加分隔符（从右到左）
    fn add_separator_to_integer(input: &str, separator: char, group_size: usize) -> String {
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

    /// 移除字符串中的分隔符
    ///
    /// # 参数
    /// * `input` - 输入字符串
    /// * `separators` - 要移除的分隔符数组
    ///
    /// # 示例
    /// ```
    /// use number_conversion::utils::formatting::Formatter;
    ///
    /// let result = Formatter::remove_separators("1234_5678 90AB", &['_', ' ']);
    /// assert_eq!(result, "1234567890AB");
    /// ```
    pub fn remove_separators(input: &str, separators: &[char]) -> String {
        input.chars().filter(|c| !separators.contains(c)).collect()
    }

    /// 格式化十六进制字符串（大写）
    ///
    /// # 示例
    /// ```
    /// use number_conversion::utils::formatting::Formatter;
    ///
    /// let result = Formatter::format_hex("abcd1234");
    /// assert_eq!(result, "ABCD1234");
    /// ```
    pub fn format_hex(input: &str) -> String {
        Self::remove_separators(input, &['_', ' ', ',']).to_uppercase()
    }

    /// 格式化二进制字符串
    ///
    /// # 示例
    /// ```
    /// use number_conversion::utils::formatting::Formatter;
    ///
    /// let result = Formatter::format_binary("10101010");
    /// assert_eq!(result, "10101010");
    /// ```
    pub fn format_binary(input: &str) -> String {
        Self::remove_separators(input, &['_', ' ', ','])
    }

    /// 格式化十进制字符串
    ///
    /// # 示例
    /// ```
    /// use number_conversion::utils::formatting::Formatter;
    ///
    /// let result = Formatter::format_decimal("1234567");
    /// assert_eq!(result, "1234567");
    /// ```
    pub fn format_decimal(input: &str) -> String {
        Self::remove_separators(input, &[',', ' ', '_'])
    }

    /// 截断长字符串并添加省略号
    ///
    /// # 参数
    /// * `input` - 输入字符串
    /// * `max_length` - 最大长度
    ///
    /// # 示例
    /// ```
    /// use number_conversion::utils::formatting::Formatter;
    ///
    /// let result = Formatter::truncate("Hello World", 8);
    /// assert_eq!(result, "Hello...");
    /// ```
    pub fn truncate(input: &str, max_length: usize) -> String {
        if input.len() <= max_length {
            input.to_string()
        } else if max_length <= 3 {
            "...".to_string()
        } else {
            format!("{}...", &input[..max_length - 3])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_separator() {
        assert_eq!(Formatter::add_separator("12345678", '_', 4), "1234_5678");
        assert_eq!(Formatter::add_separator("123", '_', 4), "123");
        assert_eq!(Formatter::add_separator("", '_', 4), "");
    }

    #[test]
    fn test_add_separator_with_decimal() {
        assert_eq!(Formatter::add_separator("12345.67", '_', 4), "1_2345.67");
    }

    #[test]
    fn test_remove_separators() {
        assert_eq!(
            Formatter::remove_separators("1234_5678 90AB", &['_', ' ']),
            "1234567890AB"
        );
    }

    #[test]
    fn test_format_hex() {
        assert_eq!(Formatter::format_hex("abcd1234"), "ABCD1234");
        assert_eq!(Formatter::format_hex("ab_cd 12_34"), "ABCD1234");
    }

    #[test]
    fn test_format_binary() {
        assert_eq!(Formatter::format_binary("10101010"), "10101010");
    }

    #[test]
    fn test_format_decimal() {
        assert_eq!(Formatter::format_decimal("1234567"), "1234567");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(Formatter::truncate("Hello World", 8), "Hello...");
        assert_eq!(Formatter::truncate("Hi", 8), "Hi");
        assert_eq!(Formatter::truncate("Hello", 3), "...");
    }
}
