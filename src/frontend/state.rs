//! Frontend state for all pages.
//!
//! This module contains all UI state structures and the main [`FrontendState`]
//! manager that coordinates between UI and backend.

use std::collections::VecDeque;

use crate::backend::{Backend, BackendRequest, BackendResponse};
use crate::backend::{
    BitViewerOperation, BitViewerRequest, CalculatorRequest, FloatConversionRequest,
    FloatConversionType, NumberConversionRequest, NumberConversionType, TextConversionRequest,
    TextConversionType,
};

/// Maximum number of history entries to keep.
const MAX_HISTORY: usize = 200;

// ============================================================================
// Number Conversion State
// ============================================================================

/// State for a single number conversion field.
#[derive(Debug, Clone)]
pub struct NumberConversionField {
    /// User input string.
    pub input: String,
    /// Binary representation of the value.
    pub binary: String,
    /// Decimal representation of the value.
    pub decimal: String,
    /// Hexadecimal representation of the value.
    pub hexadecimal: String,
    /// Error message if conversion failed.
    pub error: Option<String>,
    /// Pending request ID for async tracking.
    pub pending_id: Option<u64>,
}

impl Default for NumberConversionField {
    fn default() -> Self {
        Self {
            input: String::new(),
            binary: String::new(),
            decimal: String::new(),
            hexadecimal: String::new(),
            error: None,
            pending_id: None,
        }
    }
}

/// State for number conversion page.
#[derive(Debug, Clone, Default)]
pub struct NumberConversionState {
    /// Binary input field state.
    pub binary_field: NumberConversionField,
    /// Decimal input field state.
    pub decimal_field: NumberConversionField,
    /// Hexadecimal input field state.
    pub hex_field: NumberConversionField,
}

// ============================================================================
// Float Conversion State
// ============================================================================

/// State for float conversion field.
#[derive(Debug, Clone)]
pub struct FloatConversionField {
    /// User input string.
    pub input: String,
    /// Conversion output string.
    pub output: String,
    /// IEEE 754 analysis text (for hex to f32).
    pub analysis: Option<String>,
    /// Error message if conversion failed.
    pub error: Option<String>,
    /// Pending request ID for async tracking.
    pub pending_id: Option<u64>,
}

impl Default for FloatConversionField {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            analysis: None,
            error: None,
            pending_id: None,
        }
    }
}

/// State for float conversion page.
#[derive(Debug, Clone, Default)]
pub struct FloatConversionState {
    /// f32 to hex conversion field.
    pub f32_to_hex: FloatConversionField,
    /// Hex to f32 conversion field.
    pub hex_to_f32: FloatConversionField,
}

// ============================================================================
// Text Conversion State
// ============================================================================

/// State for text conversion field.
#[derive(Debug, Clone)]
pub struct TextConversionField {
    /// User input string.
    pub input: String,
    /// Conversion output string.
    pub output: String,
    /// Error message if conversion failed.
    pub error: Option<String>,
    /// Pending request ID for async tracking.
    pub pending_id: Option<u64>,
}

impl Default for TextConversionField {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            error: None,
            pending_id: None,
        }
    }
}

/// State for text conversion page.
#[derive(Debug, Clone, Default)]
pub struct TextConversionState {
    /// ASCII to hex conversion field.
    pub ascii_to_hex: TextConversionField,
    /// Hex to ASCII conversion field.
    pub hex_to_ascii: TextConversionField,
}

// ============================================================================
// Bit Viewer State
// ============================================================================

/// State for bit viewer page.
#[derive(Debug, Clone)]
pub struct BitViewerState {
    /// Hex input string.
    pub hex_input: String,
    /// Field width configuration string.
    pub field_widths_input: String,
    /// Parsed field widths.
    pub field_widths: Vec<usize>,
    /// Binary bits representation.
    pub binary_bits: Vec<bool>,
    /// Error message if parsing failed.
    pub error: Option<String>,
    /// Pending request ID for async tracking.
    pub pending_id: Option<u64>,
}

impl Default for BitViewerState {
    fn default() -> Self {
        Self {
            hex_input: String::new(),
            field_widths_input: "4 4 4 4 4 4 4 4".to_string(),
            field_widths: vec![4, 4, 4, 4, 4, 4, 4, 4],
            binary_bits: Vec::new(),
            error: None,
            pending_id: None,
        }
    }
}

impl BitViewerState {
    /// Parse field widths from input string.
    pub fn parse_field_widths(&mut self) {
        self.field_widths.clear();
        for part in self.field_widths_input.split_whitespace() {
            if let Ok(width) = part.parse::<usize>() {
                if width > 0 && width <= 64 {
                    self.field_widths.push(width);
                }
            }
        }
        if self.field_widths.is_empty() {
            self.field_widths = vec![4, 4, 4, 4, 4, 4, 4, 4];
        }
    }

    /// Calculate field groups for display.
    pub fn calculate_field_groups(&self) -> Vec<usize> {
        let mut groups = Vec::new();
        let total_bits = self.binary_bits.len();
        let mut remaining_bits = total_bits;

        for &width in &self.field_widths {
            if remaining_bits == 0 {
                break;
            }
            let group_size = width.min(remaining_bits);
            groups.push(group_size);
            remaining_bits -= group_size;
        }

        if remaining_bits > 0 {
            groups.push(remaining_bits);
        }

        groups
    }

    /// Calculate field value for display.
    pub fn calculate_field_value(&self, start_bit: usize, bit_count: usize) -> u64 {
        let mut value = 0u64;
        for i in 0..bit_count {
            if start_bit + i < self.binary_bits.len() {
                if self.binary_bits[start_bit + i] {
                    value |= 1 << (bit_count - 1 - i);
                }
            }
        }
        value
    }
}

// ============================================================================
// Calculator State
// ============================================================================

/// History entry for calculator.
#[derive(Clone, Debug)]
pub struct CalculatorHistoryEntry {
    /// Radix used for input.
    pub radix: u32,
    /// Original input expression.
    pub input: String,
    /// Decimal expression sent to backend.
    pub decimal_expr: String,
    /// Output/result string.
    pub output: String,
    /// Error message if evaluation failed.
    pub error: Option<String>,
}

/// State for calculator page.
#[derive(Debug, Clone)]
pub struct CalculatorState {
    /// Current radix (2, 8, 10, or 16).
    pub radix: u32,
    /// User input expression.
    pub input: String,
    /// Output/result string.
    pub output: String,
    /// Last error message.
    pub last_error: Option<String>,
    /// Last computed value.
    pub last_value: Option<f64>,
    /// History of calculations.
    pub history: VecDeque<CalculatorHistoryEntry>,
    /// Pending request ID for async tracking.
    pub pending_id: Option<u64>,
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            radix: 10,
            input: String::new(),
            output: String::new(),
            last_error: None,
            last_value: None,
            history: VecDeque::new(),
            pending_id: None,
        }
    }
}

// ============================================================================
// Frontend State Manager
// ============================================================================

/// Main frontend state that manages all pages and backend communication.
pub struct FrontendState {
    /// Number conversion page state.
    pub number_conversion: NumberConversionState,
    /// Float conversion page state.
    pub float_conversion: FloatConversionState,
    /// Text conversion page state.
    pub text_conversion: TextConversionState,
    /// Bit viewer page state.
    pub bit_viewer: BitViewerState,
    /// Calculator page state.
    pub calculator: CalculatorState,
    /// Backend communication handle.
    pub backend: Backend,
}

impl FrontendState {
    /// Create a new frontend state with default values.
    pub fn new() -> Self {
        Self {
            number_conversion: NumberConversionState::default(),
            float_conversion: FloatConversionState::default(),
            text_conversion: TextConversionState::default(),
            bit_viewer: BitViewerState::default(),
            calculator: CalculatorState::default(),
            backend: Backend::new(),
        }
    }

    /// Poll backend for responses and update state.
    pub fn poll_responses(&mut self) {
        while let Some(response) = self.backend.try_recv_response() {
            self.handle_response(response);
        }
    }

    fn handle_response(&mut self, response: BackendResponse) {
        match response {
            BackendResponse::NumberConversion(resp) => {
                let id = resp.id;
                let error = resp.error.clone();
                let binary = resp.binary.clone().unwrap_or_default();
                let decimal = resp.decimal.clone().unwrap_or_default();
                let hexadecimal = resp.hexadecimal.clone().unwrap_or_default();
                
                if self.number_conversion.binary_field.pending_id == Some(id) {
                    self.number_conversion.binary_field.pending_id = None;
                    self.number_conversion.binary_field.error = error;
                    self.number_conversion.binary_field.binary = binary.clone();
                    self.number_conversion.binary_field.decimal = decimal.clone();
                    self.number_conversion.binary_field.hexadecimal = hexadecimal.clone();
                } else if self.number_conversion.decimal_field.pending_id == Some(id) {
                    self.number_conversion.decimal_field.pending_id = None;
                    self.number_conversion.decimal_field.error = error;
                    self.number_conversion.decimal_field.binary = binary.clone();
                    self.number_conversion.decimal_field.decimal = decimal.clone();
                    self.number_conversion.decimal_field.hexadecimal = hexadecimal.clone();
                } else if self.number_conversion.hex_field.pending_id == Some(id) {
                    self.number_conversion.hex_field.pending_id = None;
                    self.number_conversion.hex_field.error = error;
                    self.number_conversion.hex_field.binary = binary;
                    self.number_conversion.hex_field.decimal = decimal;
                    self.number_conversion.hex_field.hexadecimal = hexadecimal;
                }
            }
            BackendResponse::TextConversion(resp) => {
                let id = resp.id;
                let output = resp.output.clone();
                let error = resp.error.clone();
                
                if self.text_conversion.ascii_to_hex.pending_id == Some(id) {
                    self.text_conversion.ascii_to_hex.pending_id = None;
                    self.text_conversion.ascii_to_hex.output = output;
                    self.text_conversion.ascii_to_hex.error = error;
                } else if self.text_conversion.hex_to_ascii.pending_id == Some(id) {
                    self.text_conversion.hex_to_ascii.pending_id = None;
                    self.text_conversion.hex_to_ascii.output = output;
                    self.text_conversion.hex_to_ascii.error = error;
                }
            }
            BackendResponse::FloatConversion(resp) => {
                let id = resp.id;
                let output = resp.output.clone();
                let error = resp.error.clone();
                let analysis = resp.analysis.clone();
                
                if self.float_conversion.f32_to_hex.pending_id == Some(id) {
                    self.float_conversion.f32_to_hex.pending_id = None;
                    self.float_conversion.f32_to_hex.output = output;
                    self.float_conversion.f32_to_hex.error = error;
                } else if self.float_conversion.hex_to_f32.pending_id == Some(id) {
                    self.float_conversion.hex_to_f32.pending_id = None;
                    self.float_conversion.hex_to_f32.output = output;
                    self.float_conversion.hex_to_f32.analysis = analysis;
                    self.float_conversion.hex_to_f32.error = error;
                }
            }
            BackendResponse::BitViewer(resp) => {
                if self.bit_viewer.pending_id == Some(resp.id) {
                    self.bit_viewer.pending_id = None;
                    self.bit_viewer.hex_input = resp.hex_input;
                    self.bit_viewer.binary_bits = resp.binary_bits;
                    self.bit_viewer.error = resp.error;
                }
            }
            BackendResponse::Calculator(resp) => {
                if self.calculator.pending_id == Some(resp.id) {
                    self.calculator.pending_id = None;
                    if let Some(error) = resp.error {
                        self.calculator.last_error = Some(error);
                        self.calculator.last_value = None;
                    } else if let Some(value) = resp.value {
                        self.calculator.last_value = Some(value);
                        self.calculator.last_error = None;
                        let output = Self::format_auto(value, self.calculator.radix);
                        self.calculator.output = output.clone();
                        self.calculator.history.push_back(CalculatorHistoryEntry {
                            radix: resp.radix,
                            input: resp.original_input,
                            decimal_expr: resp.decimal_expr,
                            output,
                            error: None,
                        });
                        while self.calculator.history.len() > MAX_HISTORY {
                            self.calculator.history.pop_front();
                        }
                    }
                }
            }
        }
    }

    // ========================================================================
    // Request Methods
    // ========================================================================

    /// Request number conversion for binary input.
    pub fn request_binary_conversion(&mut self) {
        let id = self.backend.next_id();
        self.number_conversion.binary_field.pending_id = Some(id);
        self.number_conversion.binary_field.error = None;
        self.backend.send_request(BackendRequest::NumberConversion(
            NumberConversionRequest {
                id,
                conversion_type: NumberConversionType::Binary,
                input: self.number_conversion.binary_field.input.clone(),
            },
        ));
    }

    /// Request number conversion for decimal input.
    pub fn request_decimal_conversion(&mut self) {
        let id = self.backend.next_id();
        self.number_conversion.decimal_field.pending_id = Some(id);
        self.number_conversion.decimal_field.error = None;
        self.backend.send_request(BackendRequest::NumberConversion(
            NumberConversionRequest {
                id,
                conversion_type: NumberConversionType::Decimal,
                input: self.number_conversion.decimal_field.input.clone(),
            },
        ));
    }

    /// Request number conversion for hex input.
    pub fn request_hex_conversion(&mut self) {
        let id = self.backend.next_id();
        self.number_conversion.hex_field.pending_id = Some(id);
        self.number_conversion.hex_field.error = None;
        self.backend.send_request(BackendRequest::NumberConversion(
            NumberConversionRequest {
                id,
                conversion_type: NumberConversionType::Hexadecimal,
                input: self.number_conversion.hex_field.input.clone(),
            },
        ));
    }

    /// Request text conversion.
    pub fn request_text_conversion(&mut self, ascii_to_hex: bool) {
        let id = self.backend.next_id();
        let field = if ascii_to_hex {
            &mut self.text_conversion.ascii_to_hex
        } else {
            &mut self.text_conversion.hex_to_ascii
        };
        field.pending_id = Some(id);
        field.error = None;

        self.backend.send_request(BackendRequest::TextConversion(
            TextConversionRequest {
                id,
                conversion_type: if ascii_to_hex {
                    TextConversionType::AsciiToHex
                } else {
                    TextConversionType::HexToAscii
                },
                input: field.input.clone(),
            },
        ));
    }

    /// Request float conversion.
    pub fn request_float_conversion(&mut self, f32_to_hex: bool) {
        let id = self.backend.next_id();
        let field = if f32_to_hex {
            &mut self.float_conversion.f32_to_hex
        } else {
            &mut self.float_conversion.hex_to_f32
        };
        field.pending_id = Some(id);
        field.error = None;

        self.backend.send_request(BackendRequest::FloatConversion(
            FloatConversionRequest {
                id,
                conversion_type: if f32_to_hex {
                    FloatConversionType::F32ToHex
                } else {
                    FloatConversionType::HexToF32
                },
                input: field.input.clone(),
            },
        ));
    }

    /// Request bit viewer to parse hex input.
    pub fn request_bit_viewer_parse(&mut self) {
        let id = self.backend.next_id();
        self.bit_viewer.pending_id = Some(id);
        self.bit_viewer.error = None;
        self.backend.send_request(BackendRequest::BitViewer(BitViewerRequest {
            id,
            operation: BitViewerOperation::ParseHex,
            hex_input: Some(self.bit_viewer.hex_input.clone()),
            current_bits: None,
        }));
    }

    /// Request to toggle a single bit.
    pub fn request_bit_toggle(&mut self, index: usize) {
        let id = self.backend.next_id();
        self.bit_viewer.pending_id = Some(id);
        self.backend.send_request(BackendRequest::BitViewer(BitViewerRequest {
            id,
            operation: BitViewerOperation::ToggleBit(index),
            hex_input: None,
            current_bits: Some(self.bit_viewer.binary_bits.clone()),
        }));
    }

    /// Request to invert all bits.
    pub fn request_bit_invert_all(&mut self) {
        let id = self.backend.next_id();
        self.bit_viewer.pending_id = Some(id);
        self.backend.send_request(BackendRequest::BitViewer(BitViewerRequest {
            id,
            operation: BitViewerOperation::InvertAll,
            hex_input: None,
            current_bits: Some(self.bit_viewer.binary_bits.clone()),
        }));
    }

    /// Request calculator evaluation.
    pub fn request_calculator_eval(&mut self, decimal_expr: String, radix: u32, original_input: String) {
        let id = self.backend.next_id();
        self.calculator.pending_id = Some(id);
        self.calculator.last_error = None;
        self.backend.send_request(BackendRequest::Calculator(CalculatorRequest {
            id,
            decimal_expr,
            radix,
            original_input,
        }));
    }

    // ========================================================================
    // Calculator Formatting Helpers
    // ========================================================================

    /// Format value automatically (integer or float) in given radix.
    fn format_auto(val: f64, radix: u32) -> String {
        let nearest = val.round();
        let tol = f64::max(1e-12, 1e-12 * nearest.abs());
        if (val - nearest).abs() <= tol && nearest.abs() <= (i128::MAX as f64) {
            return Self::format_value_in_radix(nearest as i128, radix);
        }
        Self::format_float_in_radix(val, radix, 16)
    }

    /// Format integer value in given radix.
    fn format_value_in_radix(val: i128, radix: u32) -> String {
        let neg = val < 0;
        let u = if neg { (-val) as u128 } else { val as u128 };
        let s = match radix {
            10 => u.to_string(),
            2 => Self::format_radix(u, 2),
            8 => Self::format_radix(u, 8),
            16 => Self::format_radix_hex(u),
            _ => u.to_string(),
        };
        if neg { format!("-{s}") } else { s }
    }

    /// Format unsigned integer in given radix (2-10).
    fn format_radix(mut v: u128, radix: u32) -> String {
        if v == 0 { return "0".to_string(); }
        let mut buf = Vec::new();
        while v > 0 {
            let d = (v % radix as u128) as u32;
            buf.push(char::from(b'0' + (d as u8)));
            v /= radix as u128;
        }
        buf.iter().rev().collect()
    }

    /// Format unsigned integer in hexadecimal.
    fn format_radix_hex(mut v: u128) -> String {
        if v == 0 { return "0".to_string(); }
        let mut buf = Vec::new();
        while v > 0 {
            let d = (v % 16) as u8;
            buf.push(match d {
                0..=9 => (b'0' + d) as char,
                _ => (b'A' + (d - 10)) as char,
            });
            v /= 16;
        }
        buf.iter().rev().collect()
    }

    /// Format float value in given radix with specified fraction digits.
    fn format_float_in_radix(val: f64, radix: u32, frac_digits: usize) -> String {
        if !val.is_finite() { return "NaN".to_string(); }
        if radix == 10 {
            let mut s = format!("{:.12}", val);
            if s.contains('.') {
                while s.ends_with('0') { s.pop(); }
                if s.ends_with('.') { s.pop(); }
            }
            return s;
        }

        let neg = val.is_sign_negative();
        let abs = val.abs();
        let int_part_f = abs.trunc();

        if int_part_f > (u128::MAX as f64) {
            let mut s = format!("{:.12}", val);
            if s.contains('.') {
                while s.ends_with('0') { s.pop(); }
                if s.ends_with('.') { s.pop(); }
            }
            return format!("{} (十进制)", s);
        }

        let int_u = int_part_f as u128;
        let mut int_str = match radix {
            2 => Self::format_radix(int_u, 2),
            8 => Self::format_radix(int_u, 8),
            16 => Self::format_radix_hex(int_u),
            _ => int_u.to_string(),
        };

        let frac = abs - (int_u as f64);
        if frac_digits == 0 || frac <= 0.0 {
            if neg && (int_u != 0 || frac == 0.0) { int_str = format!("-{}", int_str); }
            return int_str;
        }

        let mut frac_str = String::new();
        let r = radix as f64;
        let mut f = frac;
        for _ in 0..frac_digits {
            f *= r;
            let d = f.floor();
            let di = d as u32;
            frac_str.push(match di {
                0..=9 => (b'0' + (di as u8)) as char,
                _ => (b'A' + ((di - 10) as u8)) as char,
            });
            f -= d;
            if f < 1e-12 { break; }
        }

        let result = if frac_str.is_empty() { int_str.clone() } else { format!("{}.{}", int_str, frac_str) };
        if neg { format!("-{}", result) } else { result }
    }
}

impl Default for FrontendState {
    fn default() -> Self {
        Self::new()
    }
}
