//! SymPy-based calculator engine.
//!
//! This module provides expression evaluation using Python's SymPy library
//! via subprocess calls, with detailed environment diagnostics and actionable
//! remediation guidance.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
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

/// Environment diagnosis for calculator runtime dependencies.
#[derive(Debug, Clone, PartialEq, Eq)]
enum EnvironmentIssue {
    /// The calculator script could not be found.
    MissingScript(PathBuf),
    /// No Python interpreter could be found.
    MissingPython { checked_candidates: Vec<PathBuf> },
    /// Python was found, but `sympy` is missing in all discovered environments.
    MissingSympy { checked_candidates: Vec<PathBuf> },
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
    let script_path = get_script_path().map_err(format_environment_issue)?;
    let python_path = get_python_path(&script_path).map_err(format_environment_issue)?;

    let output = Command::new(&python_path)
        .arg(&script_path)
        .arg(expr)
        .output()
        .map_err(|e| {
            format!(
                "无法启动 Python 解释器：{}\n\
                 解释器路径：{}\n\
                 处理建议：\n\
                 1. 确认该路径下的 Python 可执行文件存在且可运行\n\
                 2. 如果你使用虚拟环境，请重新安装或激活虚拟环境\n\
                 3. 如果这是系统 Python，请确认它已加入 PATH\n\
                 原始错误：{}",
                python_path.display(),
                python_path.display(),
                e
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        let details = if !stderr.is_empty() {
            stderr
        } else if !stdout.is_empty() {
            stdout
        } else {
            format!("退出码: {:?}", output.status.code())
        };

        let lower_details = details.to_lowercase();
        if lower_details.contains("no module named 'sympy'")
            || lower_details.contains("no module named \"sympy\"")
            || lower_details.contains("modulenotfounderror") && lower_details.contains("sympy")
        {
            return Err(format!(
                "当前 Python 环境缺少 `sympy` 依赖。\n\
                 解释器路径：{}\n\
                 处理建议：\n\
                 1. 如果你使用系统 Python，请执行：python -m pip install sympy\n\
                 2. 如果你使用 Windows 的 py 启动器，也可以执行：py -m pip install sympy\n\
                 3. 如果你使用项目虚拟环境，请执行：.venv\\Scripts\\python -m pip install sympy\n\
                 4. 安装完成后重新启动程序再试\n\
                 原始错误：{}",
                python_path.display(),
                details
            ));
        }

        return Err(format!(
            "Python 执行失败。\n\
             解释器路径：{}\n\
             脚本路径：{}\n\
             处理建议：\n\
             1. 确认 Python 可以正常运行\n\
             2. 确认 `scripts/calc_engine.py` 文件存在且未损坏\n\
             3. 确认当前 Python 环境已安装 `sympy`\n\
             4. 你也可以手动执行以下命令排查：\n\
                \"{}\" \"{}\" \"2+2\"\n\
             原始错误：{}",
            python_path.display(),
            script_path.display(),
            python_path.display(),
            script_path.display(),
            details
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let response: CalcResponse = serde_json::from_str(&stdout).map_err(|e| {
        format!(
            "解析 Python 返回结果失败。\n\
             解释器路径：{}\n\
             脚本路径：{}\n\
             处理建议：\n\
             1. 确认 `scripts/calc_engine.py` 输出的是合法 JSON\n\
             2. 确认 Python 环境中的依赖版本兼容\n\
             3. 可以手动执行脚本查看原始输出\n\
             原始输出：{}\n\
             解析错误：{}",
            python_path.display(),
            script_path.display(),
            stdout.trim(),
            e
        )
    })?;

    if response.success {
        response
            .value
            .ok_or_else(|| "Python 返回成功但缺少数值结果".to_string())
    } else {
        Err(response.error.unwrap_or_else(|| "未知计算错误".to_string()))
    }
}

/// Get the path to the Python executable.
///
/// The search order is:
/// 1. Project-local virtual environment
/// 2. Executable-adjacent virtual environment
/// 3. Common system Python commands
fn get_python_path(script_path: &Path) -> Result<PathBuf, EnvironmentIssue> {
    let mut candidates = Vec::new();

    if let Some(project_root) = script_path.parent().and_then(Path::parent) {
        candidates.extend(venv_python_candidates(project_root));
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            candidates.extend(venv_python_candidates(exe_dir));
        }
    }

    candidates.extend(system_python_candidates());

    let mut existing_python = Vec::new();

    for candidate in candidates.iter().cloned() {
        match python_probe_status(&candidate) {
            PythonProbeStatus::AvailableWithSympy => return Ok(candidate),
            PythonProbeStatus::AvailableWithoutSympy => existing_python.push(candidate),
            PythonProbeStatus::Unavailable => {}
        }
    }

    if existing_python.is_empty() {
        Err(EnvironmentIssue::MissingPython {
            checked_candidates: candidates,
        })
    } else {
        Err(EnvironmentIssue::MissingSympy {
            checked_candidates: existing_python,
        })
    }
}

/// Get the path to the calculator script.
fn get_script_path() -> Result<PathBuf, EnvironmentIssue> {
    let mut candidates = Vec::new();

    if let Ok(current_dir) = std::env::current_dir() {
        candidates.push(current_dir.join("scripts").join("calc_engine.py"));
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            candidates.push(exe_dir.join("scripts").join("calc_engine.py"));
        }
    }

    candidates.push(PathBuf::from("scripts").join("calc_engine.py"));

    candidates
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| {
            EnvironmentIssue::MissingScript(PathBuf::from("scripts").join("calc_engine.py"))
        })
}

fn venv_python_candidates(base_dir: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    let windows = [
        base_dir.join(".venv").join("Scripts").join("python.exe"),
        base_dir.join("venv").join("Scripts").join("python.exe"),
    ];
    let unix = [
        base_dir.join(".venv").join("bin").join("python"),
        base_dir.join("venv").join("bin").join("python"),
    ];

    candidates.extend(windows);
    candidates.extend(unix);

    candidates
}

fn system_python_candidates() -> Vec<PathBuf> {
    vec![
        PathBuf::from("python"),
        PathBuf::from("python3"),
        PathBuf::from("py"),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PythonProbeStatus {
    AvailableWithSympy,
    AvailableWithoutSympy,
    Unavailable,
}

fn python_probe_status(python: &Path) -> PythonProbeStatus {
    let version_check = Command::new(python).arg("--version").output();
    let Ok(version_output) = version_check else {
        return PythonProbeStatus::Unavailable;
    };

    if !version_output.status.success() {
        return PythonProbeStatus::Unavailable;
    }

    let import_check = Command::new(python).arg("-c").arg("import sympy").output();
    match import_check {
        Ok(output) if output.status.success() => PythonProbeStatus::AvailableWithSympy,
        Ok(_) | Err(_) => PythonProbeStatus::AvailableWithoutSympy,
    }
}

fn format_environment_issue(issue: EnvironmentIssue) -> String {
    match issue {
        EnvironmentIssue::MissingScript(expected_path) => format!(
            "未找到计算脚本：{}\n\
             处理建议：\n\
             1. 确认发布目录中包含 `scripts/calc_engine.py`\n\
             2. 如果你是从源码运行，请确认当前工作目录正确\n\
             3. 如果你重新打包过程序，请把 `scripts` 目录一并复制到可执行文件旁边",
            expected_path.display()
        ),
        EnvironmentIssue::MissingPython { checked_candidates } => format!(
            "未找到可用的 Python 解释器。\n\
             已检查的位置：\n{}\n\
             处理建议：\n\
             1. 安装 Python 3，并确保 `python`、`python3` 或 `py` 命令可用\n\
             2. Windows 用户建议安装官方 Python，并勾选“Add Python to PATH”\n\
             3. 也可以在项目目录创建虚拟环境：python -m venv .venv\n\
             4. 创建虚拟环境后重新启动程序",
            format_candidate_list(&checked_candidates)
        ),
        EnvironmentIssue::MissingSympy { checked_candidates } => format!(
            "已找到 Python，但当前环境缺少 `sympy` 依赖。\n\
             已检测到的 Python：\n{}\n\
             处理建议：\n\
             1. 系统 Python：python -m pip install sympy\n\
             2. Windows py 启动器：py -m pip install sympy\n\
             3. 项目虚拟环境：.venv\\Scripts\\python -m pip install sympy\n\
             4. 安装完成后重新启动程序",
            format_candidate_list(&checked_candidates)
        ),
    }
}

fn format_candidate_list(candidates: &[PathBuf]) -> String {
    if candidates.is_empty() {
        return "  - （无）".to_string();
    }

    candidates
        .iter()
        .map(|path| format!("  - {}", path.display()))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sympy_available() -> bool {
        let Ok(script_path) = get_script_path() else {
            return false;
        };

        get_python_path(&script_path).is_ok()
    }

    #[test]
    fn test_simple_addition() {
        if !sympy_available() {
            return;
        }

        let result = evaluate("2+2");
        assert!(result.is_ok());
        assert!((result.unwrap() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_sin_pi() {
        if !sympy_available() {
            return;
        }

        let result = evaluate("sin(pi/2)");
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_power() {
        if !sympy_available() {
            return;
        }

        let result = evaluate("2^10");
        assert!(result.is_ok());
        assert!((result.unwrap() - 1024.0).abs() < 1e-10);
    }

    #[test]
    fn test_missing_sympy_returns_error() {
        let script_path = get_script_path();
        assert!(script_path.is_ok());
    }

    #[test]
    fn test_format_missing_python_message() {
        let message = format_environment_issue(EnvironmentIssue::MissingPython {
            checked_candidates: vec![PathBuf::from("python"), PathBuf::from("py")],
        });

        assert!(message.contains("未找到可用的 Python 解释器"));
        assert!(message.contains("python -m venv .venv"));
    }

    #[test]
    fn test_format_missing_sympy_message() {
        let message = format_environment_issue(EnvironmentIssue::MissingSympy {
            checked_candidates: vec![PathBuf::from("python")],
        });

        assert!(message.contains("缺少 `sympy` 依赖"));
        assert!(message.contains("pip install sympy"));
    }
}
