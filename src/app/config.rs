use std::sync::Arc;

/// 应用程序配置
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// 应用程序标题
    pub title: String,
    /// 初始窗口大小
    pub initial_window_size: [f32; 2],
    /// 是否启用日志
    pub enable_logging: bool,
    /// 字体配置
    pub font_config: FontConfig,
}

/// 字体配置
#[derive(Debug, Clone)]
pub struct FontConfig {
    /// 字体文件路径
    pub font_path: &'static str,
    /// 字体名称
    pub font_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "编码转换工具".to_string(),
            initial_window_size: [800.0, 600.0],
            enable_logging: false,
            font_config: FontConfig::default(),
        }
    }
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            font_path: "assets/fonts/STSong.ttf",
            font_name: "Song".to_string(),
        }
    }
}

impl AppConfig {
    /// 创建新的配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置窗口标题
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// 设置初始窗口大小
    pub fn with_window_size(mut self, width: f32, height: f32) -> Self {
        self.initial_window_size = [width, height];
        self
    }

    /// 启用日志
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.enable_logging = enable;
        self
    }

    /// 设置字体配置
    pub fn with_font_config(mut self, config: FontConfig) -> Self {
        self.font_config = config;
        self
    }
}

/// 字体管理器
pub struct FontManager;

impl FontManager {
    /// 设置自定义字体
    pub fn setup_fonts(ctx: &eframe::egui::Context, config: &FontConfig) {
        let mut fonts = eframe::egui::FontDefinitions::default();

        // 使用内嵌的字体数据
        let font_data = include_bytes!("../../assets/fonts/STSong.ttf").to_vec();

        fonts.font_data.insert(
            config.font_name.clone(),
            Arc::new(eframe::egui::FontData::from_owned(font_data)),
        );

        // 设置字体族
        fonts
            .families
            .entry(eframe::egui::FontFamily::Proportional)
            .or_default()
            .insert(0, config.font_name.clone());

        fonts
            .families
            .entry(eframe::egui::FontFamily::Monospace)
            .or_default()
            .push(config.font_name.clone());

        ctx.set_fonts(fonts);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.title, "编码转换工具");
        assert_eq!(config.initial_window_size, [800.0, 600.0]);
        assert!(!config.enable_logging);
    }

    #[test]
    fn test_app_config_builder() {
        let config = AppConfig::new()
            .with_title("Test App")
            .with_window_size(1024.0, 768.0)
            .with_logging(true);

        assert_eq!(config.title, "Test App");
        assert_eq!(config.initial_window_size, [1024.0, 768.0]);
        assert!(config.enable_logging);
    }

    #[test]
    fn test_font_config_default() {
        let font_config = FontConfig::default();
        assert_eq!(font_config.font_path, "assets/fonts/STSong.ttf");
        assert_eq!(font_config.font_name, "Song");
    }
}
