//! SymPy-based calculator engine.
//!
//! This module provides expression evaluation using Python's SymPy library
//! via subprocess calls.

use serde::{Deserialize, Serialize};
use std::process::Command;

/// Response from the Python calculator engine.
#[derive(Debug, Serialize, Deserialize)]
struct CalcResponse {
    success: bool,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    value: Option<f64>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Evaluate a mathematical expression using SymPy.
///
/// # Arguments
/// * `expr` - Mathematical expression in decimal notation
///
/// # Returns
/// * `Ok(f64)` - The calculated value
/// * `Err(String)` - Error message if evaluation fails
pub fn evaluate(expr: &str) -> Result<f64, String> {
    // Get the Python executable path
    let python_path = get_python_path();

    // Get the script path
    let script_path = get_script_path();

    // Call Python script
    let output = Command::new(&python_path)
        .arg(&script_path)
        .arg(expr)
        .output()
        .map_err(|e| format!("Failed to execute Python: {}", e))?;

    // Check for execution errors
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Python execution failed: {}", stderr));
    }

    // Parse response
    let stdout = String::from_utf8_lossy(&output.stdout);
    let response: CalcResponse =
        serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse response: {}", e))?;

    if response.success {
        response
            .value
            .ok_or_else(|| "No value in response".to_string())
    } else {
        Err(response
            .error
            .unwrap_or_else(|| "Unknown error".to_string()))
    }
}

/// Get the path to the Python executable.
fn get_python_path() -> String {
    // Try .venv first (development)
    let venv_path = std::env::current_dir()
        .map(|p| p.join(".venv/bin/python"))
        .unwrap_or_default();

    if venv_path.exists() {
        return venv_path.to_string_lossy().to_string();
    }

    // Try executable directory (production)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let local_python = exe_dir.join(".venv/bin/python");
            if local_python.exists() {
                return local_python.to_string_lossy().to_string();
            }
        }
    }

    // Fallback to system Python
    "python3".to_string()
}

/// Get the path to the calculator script.
fn get_script_path() -> String {
    // Try current directory first
    let local_script = std::env::current_dir()
        .map(|p| p.join("scripts/calc_engine.py"))
        .unwrap_or_default();

    if local_script.exists() {
        return local_script.to_string_lossy().to_string();
    }

    // Try executable directory
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_script = exe_dir.join("scripts/calc_engine.py");
            if exe_script.exists() {
                return exe_script.to_string_lossy().to_string();
            }
        }
    }

    // Fallback
    "scripts/calc_engine.py".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let result = evaluate("2+2");
        assert!(result.is_ok());
        assert!((result.unwrap() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_sin_pi() {
        let result = evaluate("sin(pi/2)");
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_power() {
        let result = evaluate("2^10");
        assert!(result.is_ok());
        assert!((result.unwrap() - 1024.0).abs() < 1e-10);
    }
}
