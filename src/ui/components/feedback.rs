//! 通用页面提示组件。
//!
//! 该模块用于统一页面中的状态提示、错误展示和说明信息样式，避免各页面重复拼装
//! `spinner`、错误颜色和帮助文案。

use eframe::egui::{self, Color32, RichText, Ui};

/// 统一的错误提示颜色。
pub const ERROR_COLOR: Color32 = Color32::from_rgb(200, 40, 40);

/// 统一的成功提示颜色。
pub const SUCCESS_COLOR: Color32 = Color32::from_rgb(30, 120, 30);

/// 统一的信息提示颜色。
pub const INFO_COLOR: Color32 = Color32::from_rgb(60, 90, 160);

/// 渲染统一的处理中提示。
pub fn show_pending(ui: &mut Ui, message: &str) {
    ui.horizontal(|ui| {
        ui.spinner();
        ui.label(RichText::new(message).color(Color32::DARK_GRAY));
    });
}

/// 渲染统一的错误提示。
///
/// 返回值表示是否实际渲染了错误。
pub fn show_error(ui: &mut Ui, error: Option<&str>) -> bool {
    if let Some(error) = error.filter(|msg| !msg.trim().is_empty()) {
        ui.colored_label(ERROR_COLOR, RichText::new(error).strong());
        true
    } else {
        false
    }
}

/// 渲染统一的成功提示。
///
/// 返回值表示是否实际渲染了提示。
pub fn show_success(ui: &mut Ui, message: Option<&str>) -> bool {
    if let Some(message) = message.filter(|msg| !msg.trim().is_empty()) {
        ui.colored_label(SUCCESS_COLOR, RichText::new(message).strong());
        true
    } else {
        false
    }
}

/// 渲染统一的信息提示。
///
/// 返回值表示是否实际渲染了提示。
pub fn show_info(ui: &mut Ui, message: Option<&str>) -> bool {
    if let Some(message) = message.filter(|msg| !msg.trim().is_empty()) {
        ui.colored_label(INFO_COLOR, message);
        true
    } else {
        false
    }
}

/// 渲染统一的说明折叠面板。
pub fn show_instructions(ui: &mut Ui, title: &str, lines: &[&str]) {
    if lines.is_empty() {
        return;
    }

    ui.collapsing(title, |ui| {
        for line in lines {
            if !line.trim().is_empty() {
                ui.label(*line);
            }
        }
    });
}

/// 渲染统一的结果标题。
pub fn section_title(ui: &mut Ui, title: &str) {
    ui.label(RichText::new(title).color(INFO_COLOR).strong());
}

/// 渲染统一的结果标签和值。
pub fn labeled_monospace(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.monospace(value);
    });
}

/// 渲染统一的分组容器。
pub fn group_panel(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui)) {
    ui.group(|ui| {
        section_title(ui, title);
        ui.add_space(4.0);
        add_contents(ui);
    });
}

/// 渲染统一的空状态提示。
pub fn show_empty_state(ui: &mut Ui, message: &str) {
    ui.add_space(4.0);
    ui.label(RichText::new(message).italics().color(Color32::GRAY));
}

/// 渲染统一的分隔说明文本。
pub fn helper_text(ui: &mut Ui, message: &str) {
    ui.label(RichText::new(message).small().color(Color32::GRAY));
}

/// 渲染统一的错误边框组。
pub fn error_frame(ui: &mut Ui, error: &str) {
    egui::Frame::group(ui.style())
        .stroke(egui::Stroke::new(1.0, ERROR_COLOR))
        .show(ui, |ui| {
            ui.colored_label(ERROR_COLOR, RichText::new(error).strong());
        });
}
