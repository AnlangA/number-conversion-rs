//! 应用程序模块
//!
//! 包含应用程序的主要结构和配置管理

/// 应用程序主体模块
pub mod application;
/// 应用程序配置模块
pub mod config;

pub use application::{Application, ApplicationBuilder};
pub use config::{AppConfig, FontConfig, FontManager};
