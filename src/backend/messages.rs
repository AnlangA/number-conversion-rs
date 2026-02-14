//! Message types for frontend-backend communication.

// ============================================================================
// Backend Request/Response Enums
// ============================================================================

/// Request sent from frontend to backend.
#[derive(Debug, Clone)]
pub enum BackendRequest {
    /// Number conversion request
    NumberConversion(NumberConversionRequest),
    /// Text conversion request
    TextConversion(TextConversionRequest),
    /// Float conversion request
    FloatConversion(FloatConversionRequest),
    /// Bit viewer operation
    BitViewer(BitViewerRequest),
    /// Calculator expression evaluation
    Calculator(CalculatorRequest),
    /// Shutdown the backend
    Shutdown,
}

/// Response sent from backend to frontend.
#[derive(Debug, Clone)]
pub enum BackendResponse {
    /// Number conversion result
    NumberConversion(NumberConversionResponse),
    /// Text conversion result
    TextConversion(TextConversionResponse),
    /// Float conversion result
    FloatConversion(FloatConversionResponse),
    /// Bit viewer result
    BitViewer(BitViewerResponse),
    /// Calculator result
    Calculator(CalculatorResponse),
}

// ============================================================================
// Number Conversion
// ============================================================================

/// Number conversion type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberConversionType {
    /// Binary to other bases
    Binary,
    /// Decimal to other bases
    Decimal,
    /// Hexadecimal to other bases
    Hexadecimal,
}

/// Number conversion request.
#[derive(Debug, Clone)]
pub struct NumberConversionRequest {
    /// Request ID
    pub id: u64,
    /// Conversion type
    pub conversion_type: NumberConversionType,
    /// Input value
    pub input: String,
}

/// Number conversion response.
#[derive(Debug, Clone)]
pub struct NumberConversionResponse {
    /// Request ID
    pub id: u64,
    /// Binary result
    pub binary: Option<String>,
    /// Decimal result
    pub decimal: Option<String>,
    /// Hexadecimal result
    pub hexadecimal: Option<String>,
    /// Error message
    pub error: Option<String>,
}

// ============================================================================
// Text Conversion
// ============================================================================

/// Text conversion type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextConversionType {
    /// ASCII to hexadecimal
    AsciiToHex,
    /// Hexadecimal to ASCII
    HexToAscii,
}

/// Text conversion request.
#[derive(Debug, Clone)]
pub struct TextConversionRequest {
    /// Request ID
    pub id: u64,
    /// Conversion type
    pub conversion_type: TextConversionType,
    /// Input value
    pub input: String,
}

/// Text conversion response.
#[derive(Debug, Clone)]
pub struct TextConversionResponse {
    /// Request ID
    pub id: u64,
    /// Output value
    pub output: String,
    /// Error message
    pub error: Option<String>,
}

// ============================================================================
// Float Conversion
// ============================================================================

/// Float conversion type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatConversionType {
    /// f32 to hexadecimal
    F32ToHex,
    /// Hexadecimal to f32
    HexToF32,
}

/// Float conversion request.
#[derive(Debug, Clone)]
pub struct FloatConversionRequest {
    /// Request ID
    pub id: u64,
    /// Conversion type
    pub conversion_type: FloatConversionType,
    /// Input value
    pub input: String,
}

/// Float conversion response.
#[derive(Debug, Clone)]
pub struct FloatConversionResponse {
    /// Request ID
    pub id: u64,
    /// Output value
    pub output: String,
    /// IEEE 754 analysis (for HexToF32)
    pub analysis: Option<String>,
    /// Error message
    pub error: Option<String>,
}

// ============================================================================
// Bit Viewer
// ============================================================================

/// Bit viewer operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitViewerOperation {
    /// Parse hex input to binary bits
    ParseHex,
    /// Toggle a bit at index
    ToggleBit(usize),
    /// Invert all bits
    InvertAll,
}

/// Bit viewer request.
#[derive(Debug, Clone)]
pub struct BitViewerRequest {
    /// Request ID
    pub id: u64,
    /// Operation
    pub operation: BitViewerOperation,
    /// Hex input (for ParseHex)
    pub hex_input: Option<String>,
    /// Current binary bits (for ToggleBit/InvertAll)
    pub current_bits: Option<Vec<bool>>,
}

/// Bit viewer response.
#[derive(Debug, Clone)]
pub struct BitViewerResponse {
    /// Request ID
    pub id: u64,
    /// Updated hex input
    pub hex_input: String,
    /// Binary bits
    pub binary_bits: Vec<bool>,
    /// Error message
    pub error: Option<String>,
}

// ============================================================================
// Calculator
// ============================================================================

/// Calculator request.
#[derive(Debug, Clone)]
pub struct CalculatorRequest {
    /// Request ID
    pub id: u64,
    /// Expression in decimal notation (already converted from source radix)
    pub decimal_expr: String,
    /// Source radix for display
    pub radix: u32,
    /// Original input expression
    pub original_input: String,
}

/// Calculator response.
#[derive(Debug, Clone)]
pub struct CalculatorResponse {
    /// Request ID
    pub id: u64,
    /// Calculated value (if successful)
    pub value: Option<f64>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Source radix
    pub radix: u32,
    /// Original input
    pub original_input: String,
    /// Decimal expression that was evaluated
    pub decimal_expr: String,
}
