use eframe::egui::{self, Context};

/// 应用程序页面枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppPage {
    /// 进制转换页面
    NumberConversion,
    /// 文本转换页面
    TextConversion,
    /// 位查看器页面
    BitViewer,
}

impl AppPage {
    /// 获取页面显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            AppPage::NumberConversion => "进制转换",
            AppPage::TextConversion => "字符转换",
            AppPage::BitViewer => "bit查看",
        }
    }

    /// 获取所有页面
    pub fn all() -> &'static [AppPage] {
        &[
            AppPage::NumberConversion,
            AppPage::TextConversion,
            AppPage::BitViewer,
        ]
    }
}

impl Default for AppPage {
    fn default() -> Self {
        AppPage::NumberConversion
    }
}

/// 导航组件
pub struct NavigationComponent {
    current_page: AppPage,
}

impl NavigationComponent {
    /// 创建新的导航组件
    pub fn new() -> Self {
        Self {
            current_page: AppPage::default(),
        }
    }

    /// 获取当前页面
    pub fn current_page(&self) -> AppPage {
        self.current_page
    }

    /// 设置当前页面
    pub fn set_current_page(&mut self, page: AppPage) {
        self.current_page = page;
    }

    /// 渲染导航栏
    pub fn render(&mut self, ctx: &Context) -> AppPage {
        let mut selected_page = self.current_page;

        egui::TopBottomPanel::top("navigation_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().button_padding = egui::vec2(12.0, 8.0);
                
                for &page in AppPage::all() {
                    let is_selected = page == self.current_page;
                    
                    // 创建按钮样式
                    let button = egui::Button::new(page.display_name())
                        .selected(is_selected);
                    
                    if ui.add(button).clicked() {
                        selected_page = page;
                    }
                }
                
                // 在右侧添加一些信息
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.hyperlink_to("GitHub", "https://github.com/AnlangA/number-conversion-rs");
                });
            });
        });

        // 更新当前页面
        if selected_page != self.current_page {
            self.current_page = selected_page;
        }

        self.current_page
    }
}

impl Default for NavigationComponent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_page_display_names() {
        assert_eq!(AppPage::NumberConversion.display_name(), "进制转换");
        assert_eq!(AppPage::TextConversion.display_name(), "字符转换");
        assert_eq!(AppPage::BitViewer.display_name(), "bit查看");
    }

    #[test]
    fn test_navigation_component() {
        let mut nav = NavigationComponent::new();
        assert_eq!(nav.current_page(), AppPage::NumberConversion);
        
        nav.set_current_page(AppPage::BitViewer);
        assert_eq!(nav.current_page(), AppPage::BitViewer);
    }
}
