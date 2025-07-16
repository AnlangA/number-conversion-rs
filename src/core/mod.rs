//! 核心业务逻辑模块
//! 
//! 包含应用程序的核心功能，包括数据模型、转换器和错误处理

/// 错误处理模块
pub mod errors;
/// 数据模型模块
pub mod models;
/// 转换器模块
pub mod converters;
/// 输入验证器模块
pub mod validators;

pub use errors::{ConversionError, ConversionResult};
pub use models::{ConversionData, BitViewerData};
pub use converters::{BaseConverter, TextConverter, FloatConverter};
