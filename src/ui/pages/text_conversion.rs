use eframe::egui::{self, Ui};
use crate::core::{ConversionData, TextConverter};
use crate::ui::components::ConverterPanel;

/// 文本转换页面
pub struct TextConversionPage {
    ascii_to_hex_data: ConversionData,
    hex_to_ascii_data: ConversionData,
}

impl TextConversionPage {
    /// 创建新的文本转换页面
    pub fn new() -> Self {
        Self {
            ascii_to_hex_data: ConversionData::new(),
            hex_to_ascii_data: ConversionData::new(),
        }
    }

    /// 渲染页面
    pub fn render(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("文本转换");
            ui.add_space(10.0);

            // ASCII转十六进制
            ConverterPanel::render_ascii_converter(
                ui,
                "ASCII → 十六进制",
                "输入ASCII文本，如: Hello",
                &mut self.ascii_to_hex_data,
                |data| TextConverter::ascii_to_hex(data),
            );

            // 十六进制转ASCII
            ConverterPanel::render_hex_text_converter(
                ui,
                "十六进制 → ASCII",
                "输入十六进制，如: 48 65 6C 6C 6F",
                &mut self.hex_to_ascii_data,
                |data| TextConverter::hex_to_ascii(data),
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

            // 使用说明
            ui.separator();
            ui.collapsing("使用说明", |ui| {
                ui.label("• ASCII转十六进制：将文本字符转换为对应的十六进制编码");
                ui.label("• 十六进制转ASCII：将十六进制编码转换为对应的文本字符");
                ui.label("• 十六进制输入支持空格分隔，如：48 65 6C 6C 6F");
                ui.label("• 不可打印字符将显示为 [0xXX] 格式");
            });
        });
    }

    /// 清除所有数据
    fn clear_all(&mut self) {
        self.ascii_to_hex_data = ConversionData::new();
        self.hex_to_ascii_data = ConversionData::new();
    }

    /// 加载示例数据
    fn load_examples(&mut self) {
        // ASCII转十六进制示例
        self.ascii_to_hex_data.set_input("Hello World!".to_string());
        let _ = TextConverter::ascii_to_hex(&mut self.ascii_to_hex_data);

        // 十六进制转ASCII示例
        self.hex_to_ascii_data.set_input("48 65 6C 6C 6F 20 57 6F 72 6C 64 21".to_string());
        let _ = TextConverter::hex_to_ascii(&mut self.hex_to_ascii_data);
    }
}

impl Default for TextConversionPage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_conversion_page_creation() {
        let page = TextConversionPage::new();
        assert_eq!(page.ascii_to_hex_data.raw_input(), "");
        assert_eq!(page.hex_to_ascii_data.raw_input(), "");
    }

    #[test]
    fn test_clear_all() {
        let mut page = TextConversionPage::new();
        page.ascii_to_hex_data.set_input("test".to_string());
        
        page.clear_all();
        assert_eq!(page.ascii_to_hex_data.raw_input(), "");
    }

    #[test]
    fn test_load_examples() {
        let mut page = TextConversionPage::new();
        page.load_examples();
        
        assert!(!page.ascii_to_hex_data.raw_input().is_empty());
        assert!(!page.hex_to_ascii_data.raw_input().is_empty());
    }
}
