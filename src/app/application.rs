use eframe::{egui, App as EframeApp};
use crate::app::config::{AppConfig, FontManager};
use crate::ui::{NavigationComponent, AppPage, NumberConversionPage, TextConversionPage, BitViewerPage};

/// 主应用程序结构
pub struct Application {
    /// 应用程序配置
    config: AppConfig,
    /// 导航组件
    navigation: NavigationComponent,
    /// 进制转换页面
    number_conversion_page: NumberConversionPage,
    /// 文本转换页面
    text_conversion_page: TextConversionPage,
    /// 位查看器页面
    bit_viewer_page: BitViewerPage,
}

impl Application {
    /// 创建新的应用程序实例
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::new()
            .with_title("编码转换工具")
            .with_window_size(800.0, 600.0);

        // 设置字体
        FontManager::setup_fonts(&cc.egui_ctx, &config.font_config);
        
        // 安装图像加载器
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Self {
            config,
            navigation: NavigationComponent::new(),
            number_conversion_page: NumberConversionPage::new(),
            text_conversion_page: TextConversionPage::new(),
            bit_viewer_page: BitViewerPage::new(),
        }
    }

    /// 获取应用程序配置
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// 渲染当前页面
    fn render_current_page(&mut self, ctx: &egui::Context) {
        let current_page = self.navigation.current_page();

        egui::CentralPanel::default().show(ctx, |ui| {
            match current_page {
                AppPage::NumberConversion => {
                    self.number_conversion_page.render(ui);
                }
                AppPage::TextConversion => {
                    self.text_conversion_page.render(ui);
                }
                AppPage::BitViewer => {
                    self.bit_viewer_page.render(ui);
                }
            }
        });
    }
}

impl EframeApp for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 设置视觉样式
        ctx.set_visuals(egui::Visuals::light());

        // 渲染导航栏
        self.navigation.render(ctx);

        // 渲染当前页面
        self.render_current_page(ctx);
    }
}

/// 应用程序构建器
pub struct ApplicationBuilder {
    config: AppConfig,
}

impl ApplicationBuilder {
    /// 创建新的应用程序构建器
    pub fn new() -> Self {
        Self {
            config: AppConfig::new(),
        }
    }

    /// 设置应用程序标题
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.config = self.config.with_title(title);
        self
    }

    /// 设置窗口大小
    pub fn with_window_size(mut self, width: f32, height: f32) -> Self {
        self.config = self.config.with_window_size(width, height);
        self
    }

    /// 启用日志
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.config = self.config.with_logging(enable);
        self
    }

    /// 构建并运行应用程序
    pub fn run(self) -> Result<(), eframe::Error> {
        if self.config.enable_logging {
            env_logger::init();
        }

        let title = self.config.title.clone();
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size(self.config.initial_window_size),
            ..Default::default()
        };

        eframe::run_native(
            &title,
            options,
            Box::new(move |cc| {
                let mut app = Application::new(cc);
                app.config = self.config;
                Ok(Box::new(app))
            }),
        )
    }
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_builder() {
        let builder = ApplicationBuilder::new()
            .with_title("Test App")
            .with_window_size(1024.0, 768.0)
            .with_logging(true);

        assert_eq!(builder.config.title, "Test App");
        assert_eq!(builder.config.initial_window_size, [1024.0, 768.0]);
        assert!(builder.config.enable_logging);
    }
}
