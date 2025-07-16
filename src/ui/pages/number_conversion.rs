use eframe::egui::{self, Ui};
use crate::core::{ConversionData, BaseConverter, FloatConverter};
use crate::ui::components::ConverterPanel;

/// 进制转换页面
pub struct NumberConversionPage {
    binary_data: ConversionData,
    decimal_data: ConversionData,
    hex_data: ConversionData,
    f32_to_hex_data: ConversionData,
    hex_to_f32_data: ConversionData,
}

impl NumberConversionPage {
    /// 创建新的进制转换页面
    pub fn new() -> Self {
        Self {
            binary_data: ConversionData::new(),
            decimal_data: ConversionData::new(),
            hex_data: ConversionData::new(),
            f32_to_hex_data: ConversionData::new(),
            hex_to_f32_data: ConversionData::new(),
        }
    }

    /// 渲染页面
    pub fn render(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("进制转换");
            ui.add_space(10.0);

            // 二进制转换器
            ConverterPanel::render_binary_converter(
                ui,
                "二进制转换",
                "输入二进制数，如: 1010",
                &mut self.binary_data,
                |data| BaseConverter::from_binary(data),
            );

            // 十进制转换器
            ConverterPanel::render_decimal_converter(
                ui,
                "十进制转换",
                "输入十进制数，如: 255",
                &mut self.decimal_data,
                |data| BaseConverter::from_decimal(data),
            );

            // 十六进制转换器
            ConverterPanel::render_hex_converter(
                ui,
                "十六进制转换",
                "输入十六进制数，如: FF",
                &mut self.hex_data,
                |data| BaseConverter::from_hexadecimal(data),
            );

            ui.separator();
            ui.heading("浮点数转换");
            ui.add_space(10.0);

            // f32转十六进制
            ConverterPanel::render_float_converter(
                ui,
                "f32 → 十六进制",
                "输入f32数值，如: 1.0",
                &mut self.f32_to_hex_data,
                |data| FloatConverter::f32_to_hex(data),
            );

            // 十六进制转f32（带分析功能）
            ConverterPanel::render_hex_analyzer_converter(
                ui,
                "十六进制 → f32",
                "输入8位十六进制，如: 3F800000",
                &mut self.hex_to_f32_data,
                |data| FloatConverter::hex_to_f32(data),
                |data| FloatConverter::analyze_f32_structure(data),
            );

            // 操作按钮
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("清除所有").clicked() {
                    self.clear_all();
                }
                
                if ui.button("加载示例").clicked() {
                    self.load_examples();
                }
            });
        });
    }

    /// 清除所有数据
    fn clear_all(&mut self) {
        self.binary_data = ConversionData::new();
        self.decimal_data = ConversionData::new();
        self.hex_data = ConversionData::new();
        self.f32_to_hex_data = ConversionData::new();
        self.hex_to_f32_data = ConversionData::new();
    }

    /// 加载示例数据
    fn load_examples(&mut self) {
        // 二进制示例
        self.binary_data.set_input("11111111".to_string());
        let _ = BaseConverter::from_binary(&mut self.binary_data);

        // 十进制示例
        self.decimal_data.set_input("255".to_string());
        let _ = BaseConverter::from_decimal(&mut self.decimal_data);

        // 十六进制示例
        self.hex_data.set_input("FF".to_string());
        let _ = BaseConverter::from_hexadecimal(&mut self.hex_data);

        // f32示例
        self.f32_to_hex_data.set_input("3.14159".to_string());
        let _ = FloatConverter::f32_to_hex(&mut self.f32_to_hex_data);

        // 十六进制转f32示例
        self.hex_to_f32_data.set_input("40490FDB".to_string());
        let _ = FloatConverter::hex_to_f32(&mut self.hex_to_f32_data);
    }
}

impl Default for NumberConversionPage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_conversion_page_creation() {
        let page = NumberConversionPage::new();
        assert_eq!(page.binary_data.raw_input(), "");
        assert_eq!(page.decimal_data.raw_input(), "");
    }

    #[test]
    fn test_clear_all() {
        let mut page = NumberConversionPage::new();
        page.binary_data.set_input("test".to_string());
        
        page.clear_all();
        assert_eq!(page.binary_data.raw_input(), "");
    }

    #[test]
    fn test_load_examples() {
        let mut page = NumberConversionPage::new();
        page.load_examples();
        
        assert!(!page.binary_data.raw_input().is_empty());
        assert!(!page.decimal_data.raw_input().is_empty());
    }
}
