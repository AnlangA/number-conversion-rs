//! Frontend state for all pages.
//!
//! This module contains all UI state structures and the main [`FrontendState`]
//! manager that coordinates between UI and backend.

use std::collections::VecDeque;

use crate::backend::{Backend, BackendRequest, BackendResponse};
use crate::backend::{
    BitViewerOperation, CalculatorRequest, FloatConversionRequest, FloatConversionType,
    NumberConversionRequest, NumberConversionType, TextConversionRequest, TextConversionType,
};
use crate::core::bit_ops;

/// Maximum number of history entries to keep.
const MAX_HISTORY: usize = 200;

// ============================================================================
// Number Conversion State
// ============================================================================

/// State for a single number conversion field.
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Default)]
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
    /// Raw hex text currently being edited by the user.
    pub hex_input: String,
    /// Last successfully parsed and normalized hex text.
    pub normalized_hex: String,
    /// Canonical bit-string storage.
    pub bit_string: String,
    /// Decimal view text synchronized with the current bit-string.
    pub decimal_input: String,
    /// Field width configuration string.
    pub field_widths_input: String,
    /// Parsed field widths.
    pub field_widths: Vec<usize>,
    /// Binary bits representation derived from `bit_string`.
    pub binary_bits: Vec<bool>,
    /// Shift count input string.
    pub shift_count_input: String,
    /// Parsed shift count.
    pub shift_count: usize,
    /// Whether shifted-out bits should be cleared instead of rotated.
    pub zero_fill_shift: bool,
    /// Undo history for bit-string states.
    pub undo_stack: Vec<String>,
    /// Redo history for bit-string states.
    pub redo_stack: Vec<String>,
    /// Error message if parsing failed.
    pub error: Option<String>,
    /// Pending request ID for async tracking.
    pub pending_id: Option<u64>,
}

impl Default for BitViewerState {
    fn default() -> Self {
        Self {
            hex_input: String::new(),
            normalized_hex: String::new(),
            bit_string: String::new(),
            decimal_input: String::new(),
            field_widths_input: "4 4 4 4 4 4 4 4".to_string(),
            field_widths: vec![4, 4, 4, 4, 4, 4, 4, 4],
            binary_bits: Vec::new(),
            shift_count_input: "1".to_string(),
            shift_count: 1,
            zero_fill_shift: true,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
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

    /// Parse shift count from input string.
    pub fn parse_shift_count(&mut self) {
        self.shift_count = self
            .shift_count_input
            .trim()
            .parse::<usize>()
            .ok()
            .filter(|count| *count > 0)
            .unwrap_or(1);
    }

    fn current_shift_mode(&self) -> bit_ops::ShiftMode {
        if self.zero_fill_shift {
            bit_ops::ShiftMode::ZeroFill
        } else {
            bit_ops::ShiftMode::Rotate
        }
    }

    fn sync_views_from_bit_string(&mut self) -> Result<(), String> {
        self.binary_bits = bit_ops::bit_string_to_bits(&self.bit_string)?;
        self.normalized_hex = bit_ops::bit_string_to_hex(&self.bit_string)?;
        self.decimal_input = self.bit_string_to_decimal_string()?;
        Ok(())
    }

    fn apply_bit_string(&mut self, bit_string: String) -> Result<(), String> {
        self.bit_string = bit_string;
        self.sync_views_from_bit_string()
    }

    fn bit_string_to_decimal_string(&self) -> Result<String, String> {
        if self.bit_string.is_empty() {
            return Ok(String::new());
        }

        u128::from_str_radix(&self.bit_string, 2)
            .map(|value| value.to_string())
            .map_err(|e| format!("二进制转十进制失败: {}", e))
    }

    fn decimal_string_to_bit_string(&self, decimal: &str) -> Result<String, String> {
        let trimmed = decimal.trim();
        if trimmed.is_empty() {
            return Err("输入为空".to_string());
        }

        let value = trimmed
            .parse::<u128>()
            .map_err(|e| format!("十进制解析失败: {}", e))?;

        let target_width = self.bit_string.len().max(1);
        let raw_bits = format!("{:b}", value);

        if raw_bits.len() > target_width {
            return Err(format!(
                "十进制数值超出当前位宽 {} 位，可表示范围不足",
                target_width
            ));
        }

        Ok(format!("{:0>width$}", raw_bits, width = target_width))
    }

    /// Return whether the bit viewer currently contains parsed bit data.
    pub fn has_bits(&self) -> bool {
        !self.bit_string.is_empty()
    }

    /// Return whether there is a previous bit state available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Return whether there is a redo bit state available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Return the preferred hexadecimal text for display in the UI.
    pub fn display_hex(&self) -> &str {
        if self.hex_input.is_empty() && !self.normalized_hex.is_empty() {
            &self.normalized_hex
        } else {
            &self.hex_input
        }
    }

    /// Clear all bit viewer data while preserving user configuration.
    pub fn clear_data(&mut self) {
        self.hex_input.clear();
        self.normalized_hex.clear();
        self.bit_string.clear();
        self.decimal_input.clear();
        self.binary_bits.clear();
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.error = None;
        self.pending_id = None;
    }

    /// Calculate field groups for display.
    pub fn calculate_field_groups(&self) -> Vec<usize> {
        bit_ops::calculate_field_groups(self.bit_string.len(), &self.field_widths)
    }

    /// Calculate field value for display.
    pub fn calculate_field_value(&self, start_bit: usize, bit_count: usize) -> u64 {
        bit_ops::calculate_field_value(&self.bit_string, start_bit, bit_count).unwrap_or(0)
    }

    /// Parse the current hex input locally.
    pub fn parse_hex_input_locally(&mut self) {
        match bit_ops::parse_hex_input(&self.hex_input) {
            Ok(parsed) => {
                self.apply_parsed_bit_string(parsed.bit_string, Some(parsed.normalized_hex), false);
            }
            Err(error) => {
                self.clear_derived_views();
                self.error = Some(error);
            }
        }
    }

    /// Parse the current bit-string input locally and synchronize all derived views.
    pub fn parse_bit_string_input_locally(&mut self) {
        let trimmed = self.bit_string.trim();
        if trimmed.is_empty() {
            self.clear_derived_views();
            self.error = Some("输入为空".to_string());
            return;
        }

        if !trimmed.bytes().all(|b| b == b'0' || b == b'1') {
            self.error = Some("位串只能包含 0 和 1".to_string());
            return;
        }

        self.apply_parsed_bit_string(trimmed.to_string(), None, true);
    }

    /// Parse the current decimal input locally and synchronize all derived views.
    pub fn parse_decimal_input_locally(&mut self) {
        match self.decimal_string_to_bit_string(&self.decimal_input) {
            Ok(next_bit_string) => {
                self.apply_parsed_bit_string(next_bit_string, None, true);
            }
            Err(error) => {
                self.error = Some(error);
            }
        }
    }

    /// Apply a bit operation locally and keep all derived views synchronized.
    pub fn apply_local_operation(&mut self, operation: BitViewerOperation) {
        let result = match operation {
            BitViewerOperation::ParseHex => {
                self.parse_hex_input_locally();
                return;
            }
            BitViewerOperation::ToggleBit(index) => bit_ops::toggle_bit(&self.bit_string, index),
            BitViewerOperation::InvertAll => bit_ops::invert_all(&self.bit_string),
            BitViewerOperation::ShiftLeft(count) => {
                bit_ops::shift_left(&self.bit_string, count, self.current_shift_mode())
            }
            BitViewerOperation::ShiftRight(count) => {
                bit_ops::shift_right(&self.bit_string, count, self.current_shift_mode())
            }
        };

        match result {
            Ok(bit_string) => {
                self.apply_parsed_bit_string(bit_string, None, true);
            }
            Err(error) => {
                self.error = Some(error);
            }
        }
    }

    fn apply_parsed_bit_string(
        &mut self,
        next_bit_string: String,
        normalized_hex: Option<String>,
        normalize_hex_input: bool,
    ) {
        if self.bit_string != next_bit_string {
            self.push_undo_state();
            self.redo_stack.clear();
        }

        if let Some(normalized_hex) = normalized_hex {
            self.normalized_hex = normalized_hex;
        }

        match self.apply_bit_string(next_bit_string) {
            Ok(()) => {
                if normalize_hex_input {
                    self.hex_input = self.normalized_hex.clone();
                }
                self.error = None;
            }
            Err(error) => {
                self.error = Some(error);
            }
        }
    }

    fn clear_derived_views(&mut self) {
        self.normalized_hex.clear();
        self.decimal_input.clear();
        self.binary_bits.clear();
        self.bit_string.clear();
    }

    /// Restore the previous bit-string state if available.
    pub fn undo(&mut self) {
        if let Some(previous) = self.undo_stack.pop() {
            self.redo_stack.push(self.bit_string.clone());
            if let Err(error) = self.apply_bit_string(previous) {
                self.error = Some(error);
            } else {
                self.hex_input = self.normalized_hex.clone();
                self.error = None;
            }
        }
    }

    /// Re-apply the next bit-string state if available.
    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.bit_string.clone());
            if let Err(error) = self.apply_bit_string(next) {
                self.error = Some(error);
            } else {
                self.hex_input = self.normalized_hex.clone();
                self.error = None;
            }
        }
    }

    fn push_undo_state(&mut self) {
        if !self.bit_string.is_empty() {
            self.undo_stack.push(self.bit_string.clone());
        }
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
                    self.bit_viewer.hex_input = resp.hex_input.clone();
                    self.bit_viewer.normalized_hex = resp.hex_input;
                    self.bit_viewer.binary_bits = resp.binary_bits;
                    self.bit_viewer.bit_string =
                        bit_ops::bits_to_bit_string(&self.bit_viewer.binary_bits);
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
        self.backend
            .send_request(BackendRequest::NumberConversion(NumberConversionRequest {
                id,
                conversion_type: NumberConversionType::Binary,
                input: self.number_conversion.binary_field.input.clone(),
            }));
    }

    /// Request number conversion for decimal input.
    pub fn request_decimal_conversion(&mut self) {
        let id = self.backend.next_id();
        self.number_conversion.decimal_field.pending_id = Some(id);
        self.number_conversion.decimal_field.error = None;
        self.backend
            .send_request(BackendRequest::NumberConversion(NumberConversionRequest {
                id,
                conversion_type: NumberConversionType::Decimal,
                input: self.number_conversion.decimal_field.input.clone(),
            }));
    }

    /// Request number conversion for hex input.
    pub fn request_hex_conversion(&mut self) {
        let id = self.backend.next_id();
        self.number_conversion.hex_field.pending_id = Some(id);
        self.number_conversion.hex_field.error = None;
        self.backend
            .send_request(BackendRequest::NumberConversion(NumberConversionRequest {
                id,
                conversion_type: NumberConversionType::Hexadecimal,
                input: self.number_conversion.hex_field.input.clone(),
            }));
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

        self.backend
            .send_request(BackendRequest::TextConversion(TextConversionRequest {
                id,
                conversion_type: if ascii_to_hex {
                    TextConversionType::AsciiToHex
                } else {
                    TextConversionType::HexToAscii
                },
                input: field.input.clone(),
            }));
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

        self.backend
            .send_request(BackendRequest::FloatConversion(FloatConversionRequest {
                id,
                conversion_type: if f32_to_hex {
                    FloatConversionType::F32ToHex
                } else {
                    FloatConversionType::HexToF32
                },
                input: field.input.clone(),
            }));
    }

    /// Request bit viewer to parse hex input.
    pub fn request_bit_viewer_parse(&mut self) {
        self.bit_viewer.pending_id = None;
        self.bit_viewer.parse_hex_input_locally();
    }

    /// Request to toggle a single bit.
    pub fn request_bit_toggle(&mut self, index: usize) {
        self.apply_local_bit_viewer_operation(BitViewerOperation::ToggleBit(index));
    }

    /// Request to invert all bits.
    pub fn request_bit_invert_all(&mut self) {
        self.apply_local_bit_viewer_operation(BitViewerOperation::InvertAll);
    }

    /// Request to shift bits left.
    pub fn request_bit_shift_left(&mut self) {
        self.apply_local_bit_viewer_operation(BitViewerOperation::ShiftLeft(
            self.bit_viewer.shift_count,
        ));
    }

    /// Request to shift bits right.
    pub fn request_bit_shift_right(&mut self) {
        self.apply_local_bit_viewer_operation(BitViewerOperation::ShiftRight(
            self.bit_viewer.shift_count,
        ));
    }

    fn apply_local_bit_viewer_operation(&mut self, operation: BitViewerOperation) {
        self.bit_viewer.pending_id = None;
        self.bit_viewer.apply_local_operation(operation);
    }

    /// Undo the last bit viewer operation.
    pub fn request_bit_undo(&mut self) {
        self.bit_viewer.pending_id = None;
        self.bit_viewer.undo();
    }

    /// Redo the last undone bit viewer operation.
    pub fn request_bit_redo(&mut self) {
        self.bit_viewer.pending_id = None;
        self.bit_viewer.redo();
    }

    /// Parse the current bit viewer bit-string input locally.
    pub fn request_bit_string_parse(&mut self) {
        self.bit_viewer.pending_id = None;
        self.bit_viewer.parse_bit_string_input_locally();
    }

    /// Parse the current bit viewer decimal input locally.
    pub fn request_bit_decimal_parse(&mut self) {
        self.bit_viewer.pending_id = None;
        self.bit_viewer.parse_decimal_input_locally();
    }

    /// Request calculator evaluation.
    pub fn request_calculator_eval(
        &mut self,
        decimal_expr: String,
        radix: u32,
        original_input: String,
    ) {
        let id = self.backend.next_id();
        self.calculator.pending_id = Some(id);
        self.calculator.last_error = None;
        self.backend
            .send_request(BackendRequest::Calculator(CalculatorRequest {
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
        if neg {
            format!("-{s}")
        } else {
            s
        }
    }

    /// Format unsigned integer in given radix (2-10).
    fn format_radix(mut v: u128, radix: u32) -> String {
        if v == 0 {
            return "0".to_string();
        }
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
        if v == 0 {
            return "0".to_string();
        }
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
        if !val.is_finite() {
            return "NaN".to_string();
        }
        if radix == 10 {
            let mut s = format!("{:.12}", val);
            if s.contains('.') {
                while s.ends_with('0') {
                    s.pop();
                }
                if s.ends_with('.') {
                    s.pop();
                }
            }
            return s;
        }

        let neg = val.is_sign_negative();
        let abs = val.abs();
        let int_part_f = abs.trunc();

        if int_part_f > (u128::MAX as f64) {
            let mut s = format!("{:.12}", val);
            if s.contains('.') {
                while s.ends_with('0') {
                    s.pop();
                }
                if s.ends_with('.') {
                    s.pop();
                }
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
            if neg && (int_u != 0 || frac == 0.0) {
                int_str = format!("-{}", int_str);
            }
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
            if f < 1e-12 {
                break;
            }
        }

        let result = if frac_str.is_empty() {
            int_str.clone()
        } else {
            format!("{}.{}", int_str, frac_str)
        };
        if neg {
            format!("-{}", result)
        } else {
            result
        }
    }
}

impl Default for FrontendState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::BitViewerOperation;

    #[test]
    fn bit_viewer_local_parse_updates_all_views() {
        let mut state = BitViewerState {
            hex_input: "a1 b2".to_string(),
            ..Default::default()
        };

        state.parse_hex_input_locally();

        assert_eq!(state.normalized_hex, "A1B2");
        assert_eq!(state.hex_input, "a1 b2");
        assert_eq!(state.bit_string, "1010000110110010");
        assert_eq!(
            state.binary_bits,
            vec![
                true, false, true, false, false, false, false, true, true, false, true, true,
                false, false, true, false,
            ]
        );
        assert!(state.error.is_none());
    }

    #[test]
    fn bit_viewer_local_parse_clears_derived_state_on_error() {
        let mut state = BitViewerState {
            hex_input: "FF".to_string(),
            ..Default::default()
        };
        state.parse_hex_input_locally();

        state.hex_input = "GG".to_string();
        state.parse_hex_input_locally();

        assert!(state.normalized_hex.is_empty());
        assert!(state.bit_string.is_empty());
        assert!(state.binary_bits.is_empty());
        assert!(state.error.is_some());
    }

    #[test]
    fn bit_viewer_shift_left_zero_fill_clears_shifted_out_bits() {
        let mut state = BitViewerState {
            hex_input: "B".to_string(),
            zero_fill_shift: true,
            shift_count_input: "1".to_string(),
            shift_count: 1,
            ..Default::default()
        };
        state.parse_hex_input_locally();

        state.apply_local_operation(BitViewerOperation::ShiftLeft(1));

        assert_eq!(state.bit_string, "0110");
        assert_eq!(state.normalized_hex, "6");
        assert_eq!(state.hex_input, "6");
        assert!(state.error.is_none());
    }

    #[test]
    fn bit_viewer_shift_left_rotate_preserves_shifted_out_bits() {
        let mut state = BitViewerState {
            hex_input: "B".to_string(),
            zero_fill_shift: false,
            shift_count_input: "1".to_string(),
            shift_count: 1,
            ..Default::default()
        };
        state.parse_hex_input_locally();

        state.apply_local_operation(BitViewerOperation::ShiftLeft(1));

        assert_eq!(state.bit_string, "0111");
        assert_eq!(state.normalized_hex, "7");
        assert_eq!(state.hex_input, "7");
        assert!(state.error.is_none());
    }

    #[test]
    fn bit_viewer_shift_right_rotate_uses_current_mode() {
        let mut state = BitViewerState {
            hex_input: "B".to_string(),
            zero_fill_shift: false,
            shift_count_input: "2".to_string(),
            shift_count: 2,
            ..Default::default()
        };
        state.parse_hex_input_locally();

        state.apply_local_operation(BitViewerOperation::ShiftRight(2));

        assert_eq!(state.bit_string, "1110");
        assert_eq!(state.normalized_hex, "E");
        assert_eq!(state.hex_input, "E");
        assert!(state.error.is_none());
    }

    #[test]
    fn bit_viewer_decimal_parse_updates_hex_and_bits() {
        let mut state = BitViewerState {
            hex_input: "0F".to_string(),
            ..Default::default()
        };
        state.parse_hex_input_locally();

        state.decimal_input = "10".to_string();
        state.parse_decimal_input_locally();

        assert_eq!(state.bit_string, "00001010");
        assert_eq!(state.normalized_hex, "0A");
        assert_eq!(state.hex_input, "0A");
        assert_eq!(state.decimal_input, "10");
        assert_eq!(state.binary_bits.len(), 8);
        assert!(state.error.is_none());
    }

    #[test]
    fn bit_viewer_bit_string_parse_updates_hex_and_decimal() {
        let mut state = BitViewerState {
            bit_string: "10101100".to_string(),
            ..Default::default()
        };

        state.parse_bit_string_input_locally();

        assert_eq!(state.normalized_hex, "AC");
        assert_eq!(state.decimal_input, "172");
        assert_eq!(
            state.binary_bits,
            vec![true, false, true, false, true, true, false, false]
        );
        assert!(state.error.is_none());
    }

    #[test]
    fn bit_viewer_bit_string_parse_rejects_invalid_characters() {
        let mut state = BitViewerState {
            bit_string: "10A1".to_string(),
            ..Default::default()
        };

        state.parse_bit_string_input_locally();

        assert!(state.error.is_some());
    }

    #[test]
    fn bit_viewer_decimal_parse_rejects_overflow_for_current_width() {
        let mut state = BitViewerState {
            hex_input: "0F".to_string(),
            ..Default::default()
        };
        state.parse_hex_input_locally();

        state.decimal_input = "16".to_string();
        state.parse_decimal_input_locally();

        assert_eq!(state.bit_string, "00010000");
        assert_eq!(state.normalized_hex, "10");
        assert_eq!(state.hex_input, "10");
        assert_eq!(state.binary_bits.len(), 8);
        assert!(state.error.is_none());
    }

    #[test]
    fn bit_viewer_decimal_parse_preserves_current_bit_width() {
        let mut state = BitViewerState {
            hex_input: "00FF".to_string(),
            ..Default::default()
        };
        state.parse_hex_input_locally();

        state.decimal_input = "10".to_string();
        state.parse_decimal_input_locally();

        assert_eq!(state.bit_string, "0000000000001010");
        assert_eq!(state.normalized_hex, "000A");
        assert_eq!(state.hex_input, "000A");
        assert_eq!(state.binary_bits.len(), 16);
        assert_eq!(state.decimal_input, "10");
        assert!(state.error.is_none());
    }
}
