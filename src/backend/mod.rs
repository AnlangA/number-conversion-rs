//! Backend module for asynchronous computation.
//!
//! This module provides a separate thread for heavy computations,
//! allowing the UI to remain responsive.

mod messages;
mod worker;

pub use messages::{
    BackendRequest, BackendResponse,
    NumberConversionRequest, NumberConversionResponse, NumberConversionType,
    TextConversionRequest, TextConversionResponse, TextConversionType,
    FloatConversionRequest, FloatConversionResponse, FloatConversionType,
    BitViewerRequest, BitViewerResponse, BitViewerOperation,
    CalculatorRequest, CalculatorResponse,
};
pub use worker::Backend;
