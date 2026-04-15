//! Core bit manipulation utilities for the bit viewer.
//!
//! This module centralizes bit-level operations around a string-based bit model.
//! The canonical in-memory representation is a binary string such as `"10110011"`.
//! Other views like hexadecimal text and `Vec<bool>` are derived from it.

/// Result of parsing hexadecimal input into normalized hexadecimal text and a
/// canonical bit string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedBits {
    /// Uppercase hexadecimal string containing only valid hex digits.
    pub normalized_hex: String,
    /// Bit sequence ordered from high bit to low bit.
    pub bit_string: String,
}

/// Shift behavior for bit movement operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftMode {
    /// Bits shifted out are discarded and new positions are filled with `0`.
    ZeroFill,
    /// Bits shifted out wrap around to the other side.
    Rotate,
}

/// Parse a hexadecimal string into normalized uppercase hex and a bit string.
///
/// Non-hex separator characters such as spaces and underscores are ignored.
/// Any other non-hex character causes an error.
///
/// # Errors
/// Returns an error when the input is empty after normalization or contains
/// invalid non-separator characters.
pub fn parse_hex_input(input: &str) -> Result<ParsedBits, String> {
    let mut normalized_hex = String::new();

    for ch in input.chars() {
        if ch.is_ascii_hexdigit() {
            normalized_hex.push(ch.to_ascii_uppercase());
        } else if ch.is_whitespace() || ch == '_' {
            continue;
        } else {
            return Err(format!("无效的十六进制字符: {}", ch));
        }
    }

    if normalized_hex.is_empty() {
        return Err("输入为空".to_string());
    }

    let bit_string = hex_to_bit_string(&normalized_hex)?;

    Ok(ParsedBits {
        normalized_hex,
        bit_string,
    })
}

/// Convert hexadecimal text to a bit string.
///
/// The output preserves nibble width, so each hex digit becomes exactly 4 bits.
pub fn hex_to_bit_string(hex: &str) -> Result<String, String> {
    let mut bit_string = String::with_capacity(hex.len() * 4);

    for ch in hex.chars() {
        let digit = ch
            .to_digit(16)
            .ok_or_else(|| format!("无效的十六进制字符: {}", ch))? as u8;

        for shift in (0..4).rev() {
            bit_string.push(if (digit & (1 << shift)) != 0 {
                '1'
            } else {
                '0'
            });
        }
    }

    Ok(bit_string)
}

/// Convert a bit string to uppercase hexadecimal text.
///
/// If the bit count is not a multiple of 4, the last nibble is padded on the
/// low side with `0`.
pub fn bit_string_to_hex(bit_string: &str) -> Result<String, String> {
    validate_bit_string(bit_string)?;

    if bit_string.is_empty() {
        return Ok(String::new());
    }

    let bytes = bit_string.as_bytes();
    let mut hex = String::with_capacity(bytes.len().div_ceil(4));
    let mut index = 0;

    while index < bytes.len() {
        let mut nibble = 0u8;

        for offset in 0..4 {
            let bit_index = index + offset;
            if bit_index < bytes.len() && bytes[bit_index] == b'1' {
                nibble |= 1 << (3 - offset);
            }
        }

        hex.push(nibble_to_hex(nibble));
        index += 4;
    }

    Ok(hex)
}

/// Convert a bit string to a boolean vector.
pub fn bit_string_to_bits(bit_string: &str) -> Result<Vec<bool>, String> {
    validate_bit_string(bit_string)?;
    Ok(bit_string.bytes().map(|b| b == b'1').collect())
}

/// Convert a boolean slice to a bit string.
pub fn bits_to_bit_string(bits: &[bool]) -> String {
    let mut bit_string = String::with_capacity(bits.len());
    for &bit in bits {
        bit_string.push(if bit { '1' } else { '0' });
    }
    bit_string
}

/// Toggle a bit at the given index in a bit string.
///
/// If the index is out of bounds, the original string is returned unchanged.
pub fn toggle_bit(bit_string: &str, index: usize) -> Result<String, String> {
    validate_bit_string(bit_string)?;

    let mut bytes = bit_string.as_bytes().to_vec();
    if let Some(bit) = bytes.get_mut(index) {
        *bit = if *bit == b'1' { b'0' } else { b'1' };
    }

    String::from_utf8(bytes).map_err(|e| format!("位字符串编码错误: {}", e))
}

/// Invert all bits in a bit string.
pub fn invert_all(bit_string: &str) -> Result<String, String> {
    validate_bit_string(bit_string)?;

    let mut result = String::with_capacity(bit_string.len());
    for ch in bit_string.bytes() {
        result.push(if ch == b'1' { '0' } else { '1' });
    }
    Ok(result)
}

/// Shift bits to the left according to the given mode.
pub fn shift_left(bit_string: &str, count: usize, mode: ShiftMode) -> Result<String, String> {
    validate_bit_string(bit_string)?;

    if bit_string.is_empty() || count == 0 {
        return Ok(bit_string.to_string());
    }

    let len = bit_string.len();
    match mode {
        ShiftMode::ZeroFill => {
            if count >= len {
                return Ok("0".repeat(len));
            }

            let mut result = String::with_capacity(len);
            result.push_str(&bit_string[count..]);
            result.push_str(&"0".repeat(count));
            Ok(result)
        }
        ShiftMode::Rotate => {
            let shift = count % len;
            if shift == 0 {
                return Ok(bit_string.to_string());
            }

            let mut result = String::with_capacity(len);
            result.push_str(&bit_string[shift..]);
            result.push_str(&bit_string[..shift]);
            Ok(result)
        }
    }
}

/// Shift bits to the right according to the given mode.
pub fn shift_right(bit_string: &str, count: usize, mode: ShiftMode) -> Result<String, String> {
    validate_bit_string(bit_string)?;

    if bit_string.is_empty() || count == 0 {
        return Ok(bit_string.to_string());
    }

    let len = bit_string.len();
    match mode {
        ShiftMode::ZeroFill => {
            if count >= len {
                return Ok("0".repeat(len));
            }

            let split = len - count;
            let mut result = String::with_capacity(len);
            result.push_str(&"0".repeat(count));
            result.push_str(&bit_string[..split]);
            Ok(result)
        }
        ShiftMode::Rotate => {
            let shift = count % len;
            if shift == 0 {
                return Ok(bit_string.to_string());
            }

            let split = len - shift;
            let mut result = String::with_capacity(len);
            result.push_str(&bit_string[split..]);
            result.push_str(&bit_string[..split]);
            Ok(result)
        }
    }
}

/// Calculate the numeric value of a bit range.
///
/// The range starts at `start_bit` and spans `bit_count` bits. Bits are read
/// from high bit to low bit within the selected range.
///
/// Out-of-bounds bits are ignored.
pub fn calculate_field_value(
    bit_string: &str,
    start_bit: usize,
    bit_count: usize,
) -> Result<u64, String> {
    validate_bit_string(bit_string)?;

    let bytes = bit_string.as_bytes();
    let mut value = 0u64;

    for i in 0..bit_count {
        if start_bit + i < bytes.len() && bytes[start_bit + i] == b'1' {
            value |= 1 << (bit_count - 1 - i);
        }
    }

    Ok(value)
}

/// Calculate display groups from configured field widths.
///
/// Remaining bits that do not fit into configured widths are returned as one
/// final group.
pub fn calculate_field_groups(bits_len: usize, field_widths: &[usize]) -> Vec<usize> {
    let mut groups = Vec::new();
    let mut remaining_bits = bits_len;

    for &width in field_widths {
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

fn validate_bit_string(bit_string: &str) -> Result<(), String> {
    if bit_string.bytes().all(|b| b == b'0' || b == b'1') {
        Ok(())
    } else {
        Err("位字符串只能包含 0 和 1".to_string())
    }
}

fn nibble_to_hex(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        10..=15 => (b'A' + (nibble - 10)) as char,
        _ => unreachable!("nibble must be in 0..=15"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex_input_ignores_spaces_and_underscores() {
        let parsed = parse_hex_input("a1 b2_c3").unwrap();
        assert_eq!(parsed.normalized_hex, "A1B2C3");
        assert_eq!(parsed.bit_string, "101000011011001011000011");
    }

    #[test]
    fn parse_hex_input_rejects_invalid_characters() {
        let err = parse_hex_input("12G4").unwrap_err();
        assert!(err.contains("无效的十六进制字符"));
    }

    #[test]
    fn hex_to_bit_string_preserves_nibble_width() {
        assert_eq!(hex_to_bit_string("0F").unwrap(), "00001111");
    }

    #[test]
    fn bit_string_to_hex_converts_full_nibbles() {
        assert_eq!(bit_string_to_hex("10101100").unwrap(), "AC");
    }

    #[test]
    fn bit_string_to_hex_converts_partial_nibble() {
        assert_eq!(bit_string_to_hex("101").unwrap(), "A");
    }

    #[test]
    fn bit_string_to_bits_converts_correctly() {
        let bits = bit_string_to_bits("1010").unwrap();
        assert_eq!(bits, vec![true, false, true, false]);
    }

    #[test]
    fn bits_to_bit_string_converts_correctly() {
        let bit_string = bits_to_bit_string(&[true, false, true, true]);
        assert_eq!(bit_string, "1011");
    }

    #[test]
    fn toggle_bit_flips_target_bit() {
        let result = toggle_bit("101", 1).unwrap();
        assert_eq!(result, "111");
    }

    #[test]
    fn invert_all_flips_every_bit() {
        let result = invert_all("1001").unwrap();
        assert_eq!(result, "0110");
    }

    #[test]
    fn shift_left_zero_fill_moves_bits_and_zero_fills() {
        let result = shift_left("1011", 1, ShiftMode::ZeroFill).unwrap();
        assert_eq!(result, "0110");
    }

    #[test]
    fn shift_right_zero_fill_moves_bits_and_zero_fills() {
        let result = shift_right("1011", 2, ShiftMode::ZeroFill).unwrap();
        assert_eq!(result, "0010");
    }

    #[test]
    fn shift_left_rotate_wraps_bits() {
        let result = shift_left("1011", 1, ShiftMode::Rotate).unwrap();
        assert_eq!(result, "0111");
    }

    #[test]
    fn shift_right_rotate_wraps_bits() {
        let result = shift_right("1011", 2, ShiftMode::Rotate).unwrap();
        assert_eq!(result, "1110");
    }

    #[test]
    fn shift_more_than_length_zero_fill_clears_bits() {
        let result = shift_left("110", 8, ShiftMode::ZeroFill).unwrap();
        assert_eq!(result, "000");

        let result = shift_right("110", 8, ShiftMode::ZeroFill).unwrap();
        assert_eq!(result, "000");
    }

    #[test]
    fn shift_more_than_length_rotate_uses_modulo() {
        let result = shift_left("1101", 5, ShiftMode::Rotate).unwrap();
        assert_eq!(result, "1011");

        let result = shift_right("1101", 5, ShiftMode::Rotate).unwrap();
        assert_eq!(result, "1110");
    }

    #[test]
    fn calculate_field_value_reads_high_to_low() {
        assert_eq!(calculate_field_value("101100", 0, 4).unwrap(), 0b1011);
        assert_eq!(calculate_field_value("101100", 2, 3).unwrap(), 0b110);
    }

    #[test]
    fn calculate_field_groups_appends_remaining_bits() {
        let groups = calculate_field_groups(18, &[4, 4, 4]);
        assert_eq!(groups, vec![4, 4, 4, 6]);
    }

    #[test]
    fn invalid_bit_string_is_rejected() {
        let err = bit_string_to_hex("10A1").unwrap_err();
        assert!(err.contains("位字符串只能包含 0 和 1"));
    }
}
