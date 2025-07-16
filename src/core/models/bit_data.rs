use crate::core::errors::{ConversionError, ConversionResult, validate_not_empty, validate_radix_chars};

/// 位查看器的数据模型
#[derive(Debug, Clone)]
pub struct BitViewerData {
    /// 十六进制输入
    hex_input: String,
    /// 字段宽度配置字符串
    field_widths_input: String,
    /// 解析后的字段宽度数组
    field_widths: Vec<usize>,
    /// 二进制位数组
    binary_bits: Vec<bool>,
    /// 最后的错误
    last_error: Option<ConversionError>,
}

impl BitViewerData {
    /// 创建新的位查看器数据
    pub fn new() -> Self {
        Self {
            hex_input: String::new(),
            field_widths_input: "4 4 4 4 4 4 4 4".to_string(),
            field_widths: vec![4, 4, 4, 4, 4, 4, 4, 4],
            binary_bits: Vec::new(),
            last_error: None,
        }
    }

    /// 设置十六进制输入
    pub fn set_hex_input(&mut self, input: String) {
        self.hex_input = input;
        self.update_binary_bits();
    }

    /// 获取十六进制输入的可变引用
    pub fn hex_input_mut(&mut self) -> &mut String {
        &mut self.hex_input
    }

    /// 获取十六进制输入
    pub fn hex_input(&self) -> &str {
        &self.hex_input
    }

    /// 设置字段宽度输入
    pub fn set_field_widths_input(&mut self, input: String) {
        self.field_widths_input = input;
        self.parse_field_widths();
    }

    /// 获取字段宽度输入的可变引用
    pub fn field_widths_input_mut(&mut self) -> &mut String {
        &mut self.field_widths_input
    }

    /// 获取字段宽度输入
    pub fn field_widths_input(&self) -> &str {
        &self.field_widths_input
    }

    /// 获取解析后的字段宽度
    pub fn field_widths(&self) -> &[usize] {
        &self.field_widths
    }

    /// 获取二进制位
    pub fn binary_bits(&self) -> &[bool] {
        &self.binary_bits
    }

    /// 切换指定位置的位值
    pub fn toggle_bit(&mut self, index: usize) {
        if index < self.binary_bits.len() {
            self.binary_bits[index] = !self.binary_bits[index];
            self.update_hex_from_bits();
        }
    }

    /// 获取最后的错误
    pub fn last_error(&self) -> Option<&ConversionError> {
        self.last_error.as_ref()
    }

    /// 检查是否有错误
    pub fn has_error(&self) -> bool {
        self.last_error.is_some()
    }

    /// 清除所有数据
    pub fn clear(&mut self) {
        self.hex_input.clear();
        self.binary_bits.clear();
        self.last_error = None;
    }

    /// 设置示例数据
    pub fn set_example(&mut self) {
        self.set_hex_input("A1B2C3D4".to_string());
    }

    /// 从十六进制输入更新二进制位
    fn update_binary_bits(&mut self) {
        self.last_error = None;

        // 清理输入
        let clean_hex = self.hex_input
            .replace(" ", "")
            .replace("_", "")
            .to_uppercase();

        if let Err(e) = self.validate_hex_input(&clean_hex) {
            self.last_error = Some(e);
            self.binary_bits.clear();
            return;
        }

        // 转换为二进制位
        self.binary_bits.clear();
        for hex_char in clean_hex.chars() {
            if let Some(digit) = hex_char.to_digit(16) {
                let digit = digit as u8;
                for i in (0..4).rev() {
                    self.binary_bits.push((digit & (1 << i)) != 0);
                }
            }
        }
    }

    /// 从二进制位更新十六进制输入
    fn update_hex_from_bits(&mut self) {
        if self.binary_bits.is_empty() {
            return;
        }

        let mut hex_string = String::new();
        let mut current_nibble = 0u8;
        
        for (i, &bit) in self.binary_bits.iter().enumerate() {
            let bit_pos = 3 - (i % 4);
            if bit {
                current_nibble |= 1 << bit_pos;
            }
            
            if (i + 1) % 4 == 0 {
                hex_string.push_str(&format!("{:X}", current_nibble));
                current_nibble = 0;
            }
        }
        
        // 处理不完整的最后一个nibble
        if self.binary_bits.len() % 4 != 0 {
            hex_string.push_str(&format!("{:X}", current_nibble));
        }
        
        self.hex_input = hex_string;
    }

    /// 解析字段宽度配置
    fn parse_field_widths(&mut self) {
        self.field_widths.clear();
        
        for part in self.field_widths_input.split_whitespace() {
            if let Ok(width) = part.parse::<usize>() {
                if width > 0 && width <= 64 {
                    self.field_widths.push(width);
                }
            }
        }
        
        // 如果解析失败，使用默认值
        if self.field_widths.is_empty() {
            self.field_widths = vec![4, 4, 4, 4, 4, 4, 4, 4];
        }
    }

    /// 验证十六进制输入
    fn validate_hex_input(&self, input: &str) -> ConversionResult<()> {
        validate_not_empty(input)?;
        validate_radix_chars(input, 16)?;
        Ok(())
    }

    /// 计算字段分组
    pub fn calculate_field_groups(&self) -> Vec<usize> {
        let mut groups = Vec::new();
        let total_bits = self.binary_bits.len();
        let mut remaining_bits = total_bits;

        // 按配置的字段宽度分组
        for &width in &self.field_widths {
            if remaining_bits == 0 {
                break;
            }
            let group_size = width.min(remaining_bits);
            groups.push(group_size);
            remaining_bits -= group_size;
        }

        // 如果还有剩余位，将它们作为一个整体显示
        if remaining_bits > 0 {
            groups.push(remaining_bits);
        }

        groups
    }
}

impl Default for BitViewerData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_binary() {
        let mut data = BitViewerData::new();
        data.set_hex_input("A1".to_string());
        
        let expected_bits = vec![true, false, true, false, false, false, false, true];
        assert_eq!(data.binary_bits(), expected_bits);
    }

    #[test]
    fn test_toggle_bit() {
        let mut data = BitViewerData::new();
        data.set_hex_input("A0".to_string());
        
        data.toggle_bit(7); // 切换最后一位
        assert_eq!(data.hex_input(), "A1");
    }

    #[test]
    fn test_field_widths_parsing() {
        let mut data = BitViewerData::new();
        data.set_field_widths_input("8 4 4".to_string());

        assert_eq!(data.field_widths(), &[8, 4, 4]);
    }

    #[test]
    fn test_calculate_field_groups_with_remaining_bits() {
        let mut data = BitViewerData::new();
        // 设置12位的数据 (3个十六进制字符)
        data.set_hex_input("ABC".to_string());
        // 设置字段宽度为只有1位
        data.set_field_widths_input("1".to_string());

        let groups = data.calculate_field_groups();
        // 应该有2个分组：第一个1位，剩余11位
        assert_eq!(groups, vec![1, 11]);
    }

    #[test]
    fn test_calculate_field_groups_exact_match() {
        let mut data = BitViewerData::new();
        // 设置8位的数据
        data.set_hex_input("AB".to_string());
        // 设置字段宽度为4 4
        data.set_field_widths_input("4 4".to_string());

        let groups = data.calculate_field_groups();
        // 应该正好匹配，没有剩余位
        assert_eq!(groups, vec![4, 4]);
    }

    #[test]
    fn test_calculate_field_groups_insufficient_bits() {
        let mut data = BitViewerData::new();
        // 设置4位的数据
        data.set_hex_input("A".to_string());
        // 设置字段宽度为8 4
        data.set_field_widths_input("8 4".to_string());

        let groups = data.calculate_field_groups();
        // 只有4位，所以只能有一个4位的分组
        assert_eq!(groups, vec![4]);
    }
}
