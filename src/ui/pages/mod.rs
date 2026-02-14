//! 页面模块
//!
//! 包含应用程序的各个页面渲染函数

mod bit_viewer;
mod calculator;
mod number_conversion;
mod text_conversion;

pub use bit_viewer::render as render_bit_viewer;
pub use calculator::render as render_calculator;
pub use number_conversion::render as render_number_conversion;
pub use text_conversion::render as render_text_conversion;
