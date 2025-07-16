//! # 编码转换工具
//! 
//! 一个用Rust和egui构建的跨平台编码转换工具，支持多种进制转换、文本编码转换和位操作查看。
//! 
//! ## 功能特性
//! 
//! - **进制转换**: 支持二进制、八进制、十进制、十六进制之间的相互转换
//! - **文本转换**: ASCII文本与十六进制编码的相互转换
//! - **浮点数转换**: IEEE 754单精度浮点数与十六进制表示的转换
//! - **位查看器**: 可视化查看和编辑二进制位，支持自定义字段分组
//! - **用户友好**: 现代化的图形界面，支持实时转换和错误提示
//! 
//! ## 模块结构
//! 
//! - [`app`]: 应用程序主体和配置管理
//! - [`core`]: 核心业务逻辑，包括数据模型、转换器和错误处理
//! - [`ui`]: 用户界面组件和页面
//! - [`utils`]: 工具函数，包括格式化和验证
//! 
//! ## 使用示例
//! 
//! ```rust,no_run
//! use number_conversion::app::ApplicationBuilder;
//! 
//! fn main() -> Result<(), eframe::Error> {
//!     ApplicationBuilder::new()
//!         .with_title("我的编码转换工具")
//!         .with_window_size(1024.0, 768.0)
//!         .with_logging(true)
//!         .run()
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]

pub mod app;
pub mod core;
pub mod ui;
pub mod utils;

// 重新导出常用类型
pub use app::{Application, ApplicationBuilder, AppConfig};
pub use core::{ConversionError, ConversionResult, ConversionData, BitViewerData};
pub use ui::{AppPage, NavigationComponent};

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 库描述
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// 获取库的完整版本信息
pub fn version_info() -> String {
    format!("{} v{} - {}", NAME, VERSION, DESCRIPTION)
}
