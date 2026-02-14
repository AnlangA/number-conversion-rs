//! Backend module for asynchronous computation.
//!
//! This module provides a separate thread for heavy computations,
//! allowing the UI to remain responsive.

mod messages;
mod worker;

pub use messages::{
    BackendRequest, BackendResponse, BitViewerOperation, BitViewerRequest, BitViewerResponse,
    CalculatorRequest, CalculatorResponse, FloatConversionRequest, FloatConversionResponse,
    FloatConversionType, NumberConversionRequest, NumberConversionResponse, NumberConversionType,
    TextConversionRequest, TextConversionResponse, TextConversionType,
};
pub use worker::Backend;
