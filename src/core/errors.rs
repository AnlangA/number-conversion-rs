use std::fmt;

/// 应用程序中可能出现的错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionError {
    /// 输入格式错误
    InvalidFormat {
        /// 期望的格式
        expected: String,
        /// 实际得到的格式
        got: String
    },
    /// 输入为空
    EmptyInput,
    /// 输入长度超出限制
    InputTooLong {
        /// 最大允许长度
        max_length: usize,
        /// 实际长度
        actual: usize
    },
    /// 数值超出范围
    ValueOutOfRange {
        /// 最小值
        min: String,
        /// 最大值
        max: String,
        /// 实际值
        value: String
    },
    /// 解析错误
    ParseError(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::InvalidFormat { expected, got } => {
                write!(f, "格式错误：期望 {}，但得到 {}", expected, got)
            }
            ConversionError::EmptyInput => write!(f, "请输入数值"),
            ConversionError::InputTooLong { max_length, actual } => {
                write!(f, "输入长度超出限制：最大 {} 位，实际 {} 位", max_length, actual)
            }
            ConversionError::ValueOutOfRange { min, max, value } => {
                write!(f, "数值超出范围：{} 不在 {} 到 {} 之间", value, min, max)
            }
            ConversionError::ParseError(msg) => write!(f, "解析错误：{}", msg),
        }
    }
}

impl std::error::Error for ConversionError {}

/// 转换结果类型
pub type ConversionResult<T> = Result<T, ConversionError>;

/// 验证输入是否为空
pub fn validate_not_empty(input: &str) -> ConversionResult<()> {
    if input.trim().is_empty() {
        Err(ConversionError::EmptyInput)
    } else {
        Ok(())
    }
}

/// 验证输入长度
pub fn validate_length(input: &str, max_length: usize) -> ConversionResult<()> {
    if input.len() > max_length {
        Err(ConversionError::InputTooLong {
            max_length,
            actual: input.len(),
        })
    } else {
        Ok(())
    }
}

/// 验证字符是否符合指定进制
pub fn validate_radix_chars(input: &str, radix: u32) -> ConversionResult<()> {
    let radix_name = match radix {
        2 => "二进制",
        8 => "八进制", 
        10 => "十进制",
        16 => "十六进制",
        _ => "未知进制",
    };

    for ch in input.chars() {
        if !ch.is_digit(radix) {
            return Err(ConversionError::InvalidFormat {
                expected: format!("{}字符", radix_name),
                got: format!("字符 '{}'", ch),
            });
        }
    }
    Ok(())
}
