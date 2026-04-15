//! Backend worker thread for asynchronous computation.

use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread::{self, JoinHandle};

use super::messages::*;
use crate::core::{bit_ops, calc_engine};

/// Backend processor that handles all computation requests.
pub struct BackendWorker;

impl BackendWorker {
    /// Process a single request and return the response.
    pub fn process_request(request: BackendRequest) -> Option<BackendResponse> {
        match request {
            BackendRequest::NumberConversion(req) => Some(BackendResponse::NumberConversion(
                Self::handle_number_conversion(req),
            )),
            BackendRequest::TextConversion(req) => Some(BackendResponse::TextConversion(
                Self::handle_text_conversion(req),
            )),
            BackendRequest::FloatConversion(req) => Some(BackendResponse::FloatConversion(
                Self::handle_float_conversion(req),
            )),
            BackendRequest::BitViewer(req) => {
                Some(BackendResponse::BitViewer(Self::handle_bit_viewer(req)))
            }
            BackendRequest::Calculator(req) => {
                Some(BackendResponse::Calculator(Self::handle_calculator(req)))
            }
            BackendRequest::Shutdown => None,
        }
    }

    fn handle_number_conversion(req: NumberConversionRequest) -> NumberConversionResponse {
        let input = req.input.replace("_", "").replace(" ", "").to_uppercase();

        if input.is_empty() {
            return NumberConversionResponse {
                id: req.id,
                binary: None,
                decimal: None,
                hexadecimal: None,
                error: Some("输入为空".to_string()),
            };
        }

        // Parse the input number
        let number = match req.conversion_type {
            NumberConversionType::Binary => {
                u64::from_str_radix(&input, 2).map_err(|e| format!("二进制解析失败: {}", e))
            }
            NumberConversionType::Decimal => input
                .parse::<u64>()
                .map_err(|e| format!("十进制解析失败: {}", e)),
            NumberConversionType::Hexadecimal => {
                u64::from_str_radix(&input, 16).map_err(|e| format!("十六进制解析失败: {}", e))
            }
        };

        match number {
            Ok(n) => NumberConversionResponse {
                id: req.id,
                binary: Some(format!("{:b}", n)),
                decimal: Some(n.to_string()),
                hexadecimal: Some(format!("{:X}", n)),
                error: None,
            },
            Err(e) => NumberConversionResponse {
                id: req.id,
                binary: None,
                decimal: None,
                hexadecimal: None,
                error: Some(e),
            },
        }
    }

    fn handle_text_conversion(req: TextConversionRequest) -> TextConversionResponse {
        match req.conversion_type {
            TextConversionType::AsciiToHex => {
                let hex_result: String = req
                    .input
                    .chars()
                    .map(|c| format!("{:02X}", c as u8))
                    .collect::<Vec<String>>()
                    .join(" ");

                TextConversionResponse {
                    id: req.id,
                    output: hex_result,
                    error: None,
                }
            }
            TextConversionType::HexToAscii => {
                let clean_hex: String = req
                    .input
                    .chars()
                    .filter(|&c| c != ' ' && c != '_')
                    .collect::<String>()
                    .to_uppercase();

                if clean_hex.is_empty() {
                    return TextConversionResponse {
                        id: req.id,
                        output: String::new(),
                        error: Some("输入为空".to_string()),
                    };
                }

                if !clean_hex.len().is_multiple_of(2) {
                    return TextConversionResponse {
                        id: req.id,
                        output: String::new(),
                        error: Some("十六进制长度必须为偶数".to_string()),
                    };
                }

                let mut ascii_result = String::new();
                for chunk in clean_hex.as_bytes().chunks(2) {
                    if let Ok(hex_str) = std::str::from_utf8(chunk) {
                        if let Ok(byte_value) = u8::from_str_radix(hex_str, 16) {
                            if byte_value.is_ascii() && (32..=126).contains(&byte_value) {
                                ascii_result.push(byte_value as char);
                            } else {
                                ascii_result.push_str(&format!("[0x{:02X}]", byte_value));
                            }
                        }
                    }
                }

                TextConversionResponse {
                    id: req.id,
                    output: ascii_result,
                    error: None,
                }
            }
        }
    }

    fn handle_float_conversion(req: FloatConversionRequest) -> FloatConversionResponse {
        let input = req.input.replace("_", "").replace(" ", "");

        match req.conversion_type {
            FloatConversionType::F32ToHex => match input.parse::<f32>() {
                Ok(float_value) => {
                    let bits = float_value.to_bits();
                    FloatConversionResponse {
                        id: req.id,
                        output: format!("{:08X}", bits),
                        analysis: None,
                        error: None,
                    }
                }
                Err(e) => FloatConversionResponse {
                    id: req.id,
                    output: String::new(),
                    analysis: None,
                    error: Some(format!("无法解析为f32: {}", e)),
                },
            },
            FloatConversionType::HexToF32 => {
                if input.len() != 8 {
                    return FloatConversionResponse {
                        id: req.id,
                        output: String::new(),
                        analysis: None,
                        error: Some("十六进制长度必须为8".to_string()),
                    };
                }

                match u32::from_str_radix(&input.to_uppercase(), 16) {
                    Ok(bits) => {
                        let float_value = f32::from_bits(bits);
                        let result = if float_value.is_nan() {
                            "NaN (Not a Number)".to_string()
                        } else if float_value.is_infinite() {
                            if float_value.is_sign_positive() {
                                "+∞ (Positive Infinity)".to_string()
                            } else {
                                "-∞ (Negative Infinity)".to_string()
                            }
                        } else {
                            float_value.to_string()
                        };

                        let analysis = format!(
                            "IEEE 754 单精度浮点数分析:\n\
                            原始十六进制: 0x{:08X}\n\
                            二进制: {:032b}\n\
                            符号位 (1位): {} ({})\n\
                            指数位 (8位): {:08b} ({})\n\
                            尾数位 (23位): {:023b} (0x{:06X})\n\
                            浮点值: {}",
                            bits,
                            bits,
                            (bits >> 31) & 1,
                            if (bits >> 31) & 1 == 0 {
                                "正数"
                            } else {
                                "负数"
                            },
                            (bits >> 23) & 0xFF,
                            (bits >> 23) & 0xFF,
                            bits & 0x7FFFFF,
                            bits & 0x7FFFFF,
                            float_value
                        );

                        FloatConversionResponse {
                            id: req.id,
                            output: result,
                            analysis: Some(analysis),
                            error: None,
                        }
                    }
                    Err(e) => FloatConversionResponse {
                        id: req.id,
                        output: String::new(),
                        analysis: None,
                        error: Some(format!("十六进制解析失败: {}", e)),
                    },
                }
            }
        }
    }

    fn handle_bit_viewer(req: BitViewerRequest) -> BitViewerResponse {
        match req.operation {
            BitViewerOperation::ParseHex => {
                let hex_input = req.hex_input.unwrap_or_default();
                match bit_ops::parse_hex_input(&hex_input) {
                    Ok(parsed) => BitViewerResponse {
                        id: req.id,
                        hex_input: parsed.normalized_hex,
                        binary_bits: bit_ops::bit_string_to_bits(&parsed.bit_string)
                            .unwrap_or_default(),
                        error: None,
                    },
                    Err(error) => BitViewerResponse {
                        id: req.id,
                        hex_input,
                        binary_bits: Vec::new(),
                        error: Some(error),
                    },
                }
            }
            BitViewerOperation::ToggleBit(index) => {
                let current_bits = req.current_bits.unwrap_or_default();
                let bit_string = bit_ops::bits_to_bit_string(&current_bits);
                let updated_bit_string =
                    bit_ops::toggle_bit(&bit_string, index).unwrap_or(bit_string);

                BitViewerResponse {
                    id: req.id,
                    hex_input: bit_ops::bit_string_to_hex(&updated_bit_string).unwrap_or_default(),
                    binary_bits: bit_ops::bit_string_to_bits(&updated_bit_string)
                        .unwrap_or_default(),
                    error: None,
                }
            }
            BitViewerOperation::InvertAll => {
                let current_bits = req.current_bits.unwrap_or_default();
                let bit_string = bit_ops::bits_to_bit_string(&current_bits);
                let updated_bit_string = bit_ops::invert_all(&bit_string).unwrap_or(bit_string);

                BitViewerResponse {
                    id: req.id,
                    hex_input: bit_ops::bit_string_to_hex(&updated_bit_string).unwrap_or_default(),
                    binary_bits: bit_ops::bit_string_to_bits(&updated_bit_string)
                        .unwrap_or_default(),
                    error: None,
                }
            }
            BitViewerOperation::ShiftLeft(count) => {
                let current_bits = req.current_bits.unwrap_or_default();
                let bit_string = bit_ops::bits_to_bit_string(&current_bits);
                let updated_bit_string =
                    bit_ops::shift_left(&bit_string, count, bit_ops::ShiftMode::ZeroFill)
                        .unwrap_or(bit_string);

                BitViewerResponse {
                    id: req.id,
                    hex_input: bit_ops::bit_string_to_hex(&updated_bit_string).unwrap_or_default(),
                    binary_bits: bit_ops::bit_string_to_bits(&updated_bit_string)
                        .unwrap_or_default(),
                    error: None,
                }
            }
            BitViewerOperation::ShiftRight(count) => {
                let current_bits = req.current_bits.unwrap_or_default();
                let bit_string = bit_ops::bits_to_bit_string(&current_bits);
                let updated_bit_string =
                    bit_ops::shift_right(&bit_string, count, bit_ops::ShiftMode::ZeroFill)
                        .unwrap_or(bit_string);

                BitViewerResponse {
                    id: req.id,
                    hex_input: bit_ops::bit_string_to_hex(&updated_bit_string).unwrap_or_default(),
                    binary_bits: bit_ops::bit_string_to_bits(&updated_bit_string)
                        .unwrap_or_default(),
                    error: None,
                }
            }
        }
    }

    fn handle_calculator(req: CalculatorRequest) -> CalculatorResponse {
        match calc_engine::evaluate(&req.decimal_expr) {
            Ok(value) => {
                if !value.is_finite() {
                    CalculatorResponse {
                        id: req.id,
                        value: None,
                        error: Some("计算结果非有限数".to_string()),
                        radix: req.radix,
                        original_input: req.original_input,
                        decimal_expr: req.decimal_expr,
                    }
                } else {
                    CalculatorResponse {
                        id: req.id,
                        value: Some(value),
                        error: None,
                        radix: req.radix,
                        original_input: req.original_input,
                        decimal_expr: req.decimal_expr,
                    }
                }
            }
            Err(e) => CalculatorResponse {
                id: req.id,
                value: None,
                error: Some(e),
                radix: req.radix,
                original_input: req.original_input,
                decimal_expr: req.decimal_expr,
            },
        }
    }
}

/// Backend handle for managing the worker thread.
pub struct Backend {
    /// Request sender
    request_tx: Sender<BackendRequest>,
    /// Response receiver
    response_rx: Receiver<BackendResponse>,
    /// Worker thread handle
    _handle: Option<JoinHandle<()>>,
    /// Next request ID
    next_id: u64,
}

impl Backend {
    /// Create a new backend with a worker thread.
    pub fn new() -> Self {
        let (request_tx, request_rx) = std::sync::mpsc::channel();
        let (response_tx, response_rx) = std::sync::mpsc::channel();

        let handle = thread::spawn(move || {
            while let Ok(request) = request_rx.recv() {
                if matches!(request, BackendRequest::Shutdown) {
                    break;
                }
                if let Some(response) = BackendWorker::process_request(request) {
                    if response_tx.send(response).is_err() {
                        break;
                    }
                }
            }
        });

        Self {
            request_tx,
            response_rx,
            _handle: Some(handle),
            next_id: 0,
        }
    }

    /// Get next request ID.
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }

    /// Send a request to the backend.
    pub fn send_request(&self, request: BackendRequest) -> bool {
        self.request_tx.send(request).is_ok()
    }

    /// Try to receive a response (non-blocking).
    pub fn try_recv_response(&self) -> Option<BackendResponse> {
        match self.response_rx.try_recv() {
            Ok(response) => Some(response),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => None,
        }
    }

    /// Shutdown the backend.
    pub fn shutdown(&self) {
        let _ = self.request_tx.send(BackendRequest::Shutdown);
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        self.shutdown();
    }
}
