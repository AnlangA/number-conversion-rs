//! UI组件模块
//! 
//! 包含可复用的UI组件

/// 导航组件
pub mod navigation;
/// 转换器面板组件
pub mod converter_panel;

pub use navigation::{NavigationComponent, AppPage};
pub use converter_panel::ConverterPanel;
