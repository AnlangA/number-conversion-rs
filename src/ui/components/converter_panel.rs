use eframe::egui::{self, Color32, RichText, TextEdit, Ui};
use crate::core::{ConversionData, ConversionError};
use crate::core::validators::{
    BinaryValidator, DecimalValidator, HexValidator, FloatValidator,
    HexTextValidator, AsciiValidator
};

/// 转换器面板组件
pub struct ConverterPanel;

impl ConverterPanel {
    /// 渲染二进制转换器面板
    pub fn render_binary_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let validation_result = BinaryValidator::validate(&input);
                        data.set_input_with_validation_result(validation_result);

                        // 如果输入有效且不为空，执行转换
                        if !data.has_error() && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                if let Some(error) = data.last_error() {
                    ui.colored_label(Color32::RED, error.to_string());
                } else if !data.output().is_empty() {
                    ui.label("结果:");

                    // 多行显示结果
                    for line in data.output().lines() {
                        ui.horizontal(|ui| {
                            if line.contains(':') {
                                let parts: Vec<&str> = line.splitn(2, ':').collect();
                                if parts.len() == 2 {
                                    ui.label(RichText::new(format!("{}:", parts[0])).color(Color32::BLUE));
                                    ui.monospace(parts[1].trim());
                                }
                            } else {
                                ui.monospace(line);
                            }
                        });
                    }
                }
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染十进制转换器面板
    pub fn render_decimal_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let validation_result = DecimalValidator::validate(&input);
                        data.set_input_with_validation_result(validation_result);

                        // 如果输入有效且不为空，执行转换
                        if !data.has_error() && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                if let Some(error) = data.last_error() {
                    ui.colored_label(Color32::RED, error.to_string());
                } else if !data.output().is_empty() {
                    ui.label("结果:");

                    // 多行显示结果
                    for line in data.output().lines() {
                        ui.horizontal(|ui| {
                            if line.contains(':') {
                                let parts: Vec<&str> = line.splitn(2, ':').collect();
                                if parts.len() == 2 {
                                    ui.label(RichText::new(format!("{}:", parts[0])).color(Color32::BLUE));
                                    ui.monospace(parts[1].trim());
                                }
                            } else {
                                ui.monospace(line);
                            }
                        });
                    }
                }
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染十六进制转换器面板
    pub fn render_hex_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let validation_result = HexValidator::validate(&input);
                        data.set_input_with_validation_result(validation_result);

                        // 如果输入有效且不为空，执行转换
                        if !data.has_error() && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                if let Some(error) = data.last_error() {
                    ui.colored_label(Color32::RED, error.to_string());
                } else if !data.output().is_empty() {
                    ui.label("结果:");

                    // 多行显示结果
                    for line in data.output().lines() {
                        ui.horizontal(|ui| {
                            if line.contains(':') {
                                let parts: Vec<&str> = line.splitn(2, ':').collect();
                                if parts.len() == 2 {
                                    ui.label(RichText::new(format!("{}:", parts[0])).color(Color32::BLUE));
                                    ui.monospace(parts[1].trim());
                                }
                            } else {
                                ui.monospace(line);
                            }
                        });
                    }
                }
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染带分析功能的十六进制转换器面板
    pub fn render_hex_analyzer_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
        analyzer_fn: impl FnOnce(&mut ConversionData) -> Result<String, ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let validation_result = HexValidator::validate(&input);
                        data.set_input_with_validation_result(validation_result);

                        // 如果输入有效且不为空，执行转换
                        if !data.has_error() && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                if let Some(error) = data.last_error() {
                    ui.colored_label(Color32::RED, error.to_string());
                } else if !data.output().is_empty() {
                    ui.horizontal(|ui| {
                        ui.label("结果:");
                        ui.monospace(data.output());
                    });

                    // 分析按钮
                    ui.horizontal(|ui| {
                        if ui.button("详细分析").clicked() {
                            match analyzer_fn(data) {
                                Ok(analysis) => {
                                    data.set_analysis(analysis);
                                }
                                Err(error) => {
                                    data.set_analysis(format!("分析失败: {}", error));
                                }
                            }
                        }

                        if data.analysis().is_some() && ui.button("清除分析").clicked() {
                            data.clear_analysis();
                        }
                    });

                    // 显示分析结果
                    if let Some(analysis) = data.analysis() {
                        ui.separator();
                        ui.label(RichText::new("详细分析:").color(Color32::DARK_GREEN));

                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                for line in analysis.lines() {
                                    ui.monospace(line);
                                }
                            });
                    }
                }
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染浮点数转换器面板
    pub fn render_float_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let validation_result = FloatValidator::validate(&input);
                        data.set_input_with_validation_result(validation_result);

                        // 如果输入有效且不为空，执行转换
                        if !data.has_error() && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                ui.horizontal(|ui| {
                    if let Some(error) = data.last_error() {
                        ui.colored_label(Color32::RED, error.to_string());
                    } else if !data.output().is_empty() {
                        ui.label("结果:");
                        ui.monospace(data.output());
                    }
                });
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染ASCII文本转换器面板
    pub fn render_ascii_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let validation_result = AsciiValidator::validate(&input);
                        data.set_input_with_validation_result(validation_result);

                        // 如果输入有效且不为空，执行转换
                        if !data.has_error() && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                ui.horizontal(|ui| {
                    if let Some(error) = data.last_error() {
                        ui.colored_label(Color32::RED, error.to_string());
                    } else if !data.output().is_empty() {
                        ui.label("结果:");
                        ui.monospace(data.output());
                    }
                });
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染十六进制文本转换器面板（用于十六进制转ASCII）
    pub fn render_hex_text_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let validation_result = HexTextValidator::validate(&input);
                        data.set_input_with_validation_result(validation_result);

                        // 如果输入有效且不为空，执行转换
                        if !data.has_error() && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                ui.horizontal(|ui| {
                    if let Some(error) = data.last_error() {
                        ui.colored_label(Color32::RED, error.to_string());
                    } else if !data.output().is_empty() {
                        ui.label("结果:");
                        ui.monospace(data.output());
                    }
                });
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染带输入验证的转换器面板
    pub fn render_validated_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        radix: u32,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let is_valid = data.set_input_with_validation(input, radix);

                        // 如果输入有效且不为空，执行转换
                        if is_valid && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                ui.horizontal(|ui| {
                    if let Some(error) = data.last_error() {
                        ui.colored_label(Color32::RED, error.to_string());
                    } else if !data.output().is_empty() {
                        ui.label("结果:");
                        ui.monospace(data.format_output_with_separator());
                    }
                });
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染基础转换器面板
    pub fn render_basic_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());
                
                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );
                    
                    if response.changed() {
                        data.update_cleaned_input();
                        // 执行转换
                        if let Err(error) = converter_fn(data) {
                            data.set_error(error);
                        }
                    }
                });
                
                // 显示结果或错误
                ui.horizontal(|ui| {
                    if let Some(error) = data.last_error() {
                        ui.colored_label(Color32::RED, error.to_string());
                    } else if !data.output().is_empty() {
                        ui.label("结果:");
                        ui.monospace(data.format_output_with_separator());
                    }
                });
            });
        });
        
        ui.add_space(10.0);
    }

    /// 渲染带输入验证的多行输出转换器面板
    pub fn render_validated_multiline_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        radix: u32,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let is_valid = data.set_input_with_validation(input, radix);

                        // 如果输入有效且不为空，执行转换
                        if is_valid && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                if let Some(error) = data.last_error() {
                    ui.colored_label(Color32::RED, error.to_string());
                } else if !data.output().is_empty() {
                    ui.label("结果:");

                    // 多行显示结果
                    for line in data.output().lines() {
                        ui.horizontal(|ui| {
                            if line.contains(':') {
                                let parts: Vec<&str> = line.splitn(2, ':').collect();
                                if parts.len() == 2 {
                                    ui.label(RichText::new(format!("{}:", parts[0])).color(Color32::BLUE));
                                    ui.monospace(parts[1].trim());
                                }
                            } else {
                                ui.monospace(line);
                            }
                        });
                    }
                }
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染多行输出的转换器面板
    pub fn render_multiline_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());
                
                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );
                    
                    if response.changed() {
                        data.update_cleaned_input();
                        // 执行转换
                        if let Err(error) = converter_fn(data) {
                            data.set_error(error);
                        }
                    }
                });
                
                // 显示结果或错误
                if let Some(error) = data.last_error() {
                    ui.colored_label(Color32::RED, error.to_string());
                } else if !data.output().is_empty() {
                    ui.label("结果:");
                    
                    // 多行显示结果
                    for line in data.output().lines() {
                        ui.horizontal(|ui| {
                            if line.contains(':') {
                                let parts: Vec<&str> = line.splitn(2, ':').collect();
                                if parts.len() == 2 {
                                    ui.label(RichText::new(format!("{}:", parts[0])).color(Color32::BLUE));
                                    ui.monospace(parts[1].trim());
                                }
                            } else {
                                ui.monospace(line);
                            }
                        });
                    }
                }
            });
        });
        
        ui.add_space(10.0);
    }

    /// 渲染带浮点数验证的转换器面板
    pub fn render_float_validated_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let is_valid = data.set_input_with_float_validation(input);

                        // 如果输入有效且不为空，执行转换
                        if is_valid && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                ui.horizontal(|ui| {
                    if let Some(error) = data.last_error() {
                        ui.colored_label(Color32::RED, error.to_string());
                    } else if !data.output().is_empty() {
                        ui.label("结果:");
                        ui.monospace(data.output());
                    }
                });
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染带验证和分析功能的转换器面板
    pub fn render_validated_analyzer_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        radix: u32,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
        analyzer_fn: impl FnOnce(&mut ConversionData) -> Result<String, ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());

                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );

                    if response.changed() {
                        let input = data.raw_input().to_string();
                        let is_valid = data.set_input_with_validation(input, radix);

                        // 如果输入有效且不为空，执行转换
                        if is_valid && !data.cleaned_input().is_empty() {
                            if let Err(error) = converter_fn(data) {
                                data.set_error(error);
                            }
                        }
                    }
                });

                // 显示结果或错误
                if let Some(error) = data.last_error() {
                    ui.colored_label(Color32::RED, error.to_string());
                } else if !data.output().is_empty() {
                    ui.horizontal(|ui| {
                        ui.label("结果:");
                        ui.monospace(data.output());
                    });

                    // 分析按钮
                    ui.horizontal(|ui| {
                        if ui.button("详细分析").clicked() {
                            match analyzer_fn(data) {
                                Ok(analysis) => {
                                    data.set_analysis(analysis);
                                }
                                Err(error) => {
                                    data.set_analysis(format!("分析失败: {}", error));
                                }
                            }
                        }

                        if data.analysis().is_some() && ui.button("清除分析").clicked() {
                            data.clear_analysis();
                        }
                    });

                    // 显示分析结果
                    if let Some(analysis) = data.analysis() {
                        ui.separator();
                        ui.label(RichText::new("详细分析:").color(Color32::DARK_GREEN));

                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                for line in analysis.lines() {
                                    ui.monospace(line);
                                }
                            });
                    }
                }
            });
        });

        ui.add_space(10.0);
    }

    /// 渲染带分析功能的转换器面板
    pub fn render_analyzer_converter(
        ui: &mut Ui,
        title: &str,
        hint: &str,
        data: &mut ConversionData,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
        analyzer_fn: impl FnOnce(&mut ConversionData) -> Result<String, ConversionError>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // 标题
                ui.label(RichText::new(title).color(Color32::BLUE).strong());
                
                // 输入框
                ui.horizontal(|ui| {
                    ui.label("输入:");
                    let response = ui.add(
                        TextEdit::singleline(data.raw_input_mut())
                            .desired_width(300.0)
                            .hint_text(hint)
                    );
                    
                    if response.changed() {
                        data.update_cleaned_input();
                        // 执行转换
                        if let Err(error) = converter_fn(data) {
                            data.set_error(error);
                        }
                    }
                });
                
                // 显示结果或错误
                if let Some(error) = data.last_error() {
                    ui.colored_label(Color32::RED, error.to_string());
                } else if !data.output().is_empty() {
                    ui.horizontal(|ui| {
                        ui.label("结果:");
                        ui.monospace(data.output());
                    });
                    
                    // 分析按钮
                    ui.horizontal(|ui| {
                        if ui.button("详细分析").clicked() {
                            match analyzer_fn(data) {
                                Ok(analysis) => {
                                    data.set_analysis(analysis);
                                }
                                Err(error) => {
                                    data.set_analysis(format!("分析失败: {}", error));
                                }
                            }
                        }

                        if data.analysis().is_some() && ui.button("清除分析").clicked() {
                            data.clear_analysis();
                        }
                    });

                    // 显示分析结果
                    if let Some(analysis) = data.analysis() {
                        ui.separator();
                        ui.label(RichText::new("详细分析:").color(Color32::DARK_GREEN));

                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                for line in analysis.lines() {
                                    ui.monospace(line);
                                }
                            });
                    }
                }
            });
        });
        
        ui.add_space(10.0);
    }

    /// 渲染清除和示例按钮
    pub fn render_action_buttons(
        ui: &mut Ui,
        data: &mut ConversionData,
        example_value: &str,
        converter_fn: impl FnOnce(&mut ConversionData) -> Result<(), ConversionError>,
    ) {
        ui.horizontal(|ui| {
            if ui.button("清除").clicked() {
                *data = ConversionData::new();
            }
            
            if ui.button("示例").clicked() {
                data.set_input(example_value.to_string());
                if let Err(error) = converter_fn(data) {
                    data.set_error(error);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::converters::BaseConverter;

    #[test]
    fn test_converter_panel_basic_functionality() {
        // 这里主要测试数据流，UI测试需要在集成测试中进行
        let mut data = ConversionData::new();
        data.set_input("1010".to_string());
        
        let result = BaseConverter::from_binary(&mut data);
        assert!(result.is_ok());
        assert!(!data.output().is_empty());
    }
}
