//! UI组件模块
//!
//! 包含可复用的UI组件

/// 通用页面提示组件
pub mod feedback;
/// 导航组件
pub mod navigation;

pub use feedback::*;
pub use navigation::{AppPage, NavigationComponent};
