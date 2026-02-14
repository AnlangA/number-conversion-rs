//! # 编码转换工具
//!
//! 一个用Rust和egui构建的跨平台编码转换工具，支持多种进制转换、文本编码转换和位操作查看。
//!
//! ## 架构
//!
//! - **backend**: 后端计算引擎，运行在独立线程
//! - **frontend**: 前端状态管理，与后端异步通信
//! - **ui**: 用户界面组件和页面
//! - **core**: 核心业务逻辑（计算引擎、错误处理）
//!
//! ## 功能特性
//!
//! - **进制转换**: 支持二进制、八进制、十进制、十六进制之间的相互转换
//! - **文本转换**: ASCII文本与十六进制编码的相互转换
//! - **浮点数转换**: IEEE 754单精度浮点数与十六进制表示的转换
//! - **位查看器**: 可视化查看和编辑二进制位，支持自定义字段分组
//! - **计算器**: 多进制数学表达式计算（使用 SymPy 后端）
//! - **用户友好**: 现代化的图形界面，支持实时转换和错误提示

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]

pub mod app;
pub mod backend;
pub mod core;
pub mod frontend;
pub mod ui;

// 重新导出常用类型
pub use app::{AppConfig, Application, ApplicationBuilder};
pub use backend::Backend;
pub use frontend::FrontendState;
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
