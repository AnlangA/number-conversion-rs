//! 页面模块
//! 
//! 包含应用程序的各个页面

/// 进制转换页面
pub mod number_conversion;
/// 文本转换页面
pub mod text_conversion;
/// 位查看器页面
pub mod bit_viewer;

pub use number_conversion::NumberConversionPage;
pub use text_conversion::TextConversionPage;
pub use bit_viewer::BitViewerPage;
