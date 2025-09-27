use eframe::egui::{self, Color32, RichText, Ui, TextEdit, FontId};
use eframe::egui::text::{LayoutJob, TextFormat};
use mathcore::MathCore;
use std::collections::VecDeque;

const MAX_HISTORY: usize = 200;
const FRACTION_DIGITS: usize = 16;

/// 计算页面（包含多个子项目，目前仅实现“基数”）
pub struct CalculatorPage {
    current_tab: CalcTab,
    radix_calc: RadixCalculator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CalcTab {
    /// 基数（按指定进制输入表达式并计算）
    Radix,
}

impl CalculatorPage {
    /// 创建新的计算页面
    pub fn new() -> Self {
        Self {
            current_tab: CalcTab::Radix,
            radix_calc: RadixCalculator::new(),
        }
    }

    /// 渲染页面
    pub fn render(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(6.0);

            // 顶部子项目切换（当前只有“基数”）
            ui.horizontal(|ui| {
                let selected = matches!(self.current_tab, CalcTab::Radix);
                let button = egui::Button::new("基数").selected(selected);
                if ui.add(button).clicked() {
                    self.current_tab = CalcTab::Radix;
                }
            });

            ui.separator();

            match self.current_tab {
                CalcTab::Radix => self.radix_calc.render(ui),
            }
        });
    }
}

impl Default for CalculatorPage { fn default() -> Self { Self::new() } }

/// “基数”子项目：按选定进制输入表达式 → 使用 mathcore 计算 → 按进制输出
struct RadixCalculator {
    radix: u32,
    input: String,
    output: String,
    last_error: Option<String>,
    last_value: Option<f64>,
    history: VecDeque<HistoryEntry>,
}

#[derive(Clone, Debug)]
struct HistoryEntry {
    radix: u32,
    input: String,
    decimal_expr: String,
    output: String,
    error: Option<String>,
}

impl RadixCalculator {
    fn new() -> Self {
        Self { radix: 10, input: String::new(), output: String::new(), last_error: None, last_value: None, history: VecDeque::new() }
    }

    fn render(&mut self, ui: &mut Ui) {
        // 基数选择 + 输入框（实时计算）
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("")
                .selected_text(match self.radix { 2 => "二进制(2)", 8 => "八进制(8)", 10 => "十进制(10)", 16 => "十六进制(16)", _ => "自定义" })
                .show_ui(ui, |ui| {
                    for (r, name) in [(2, "二进制(2)"), (8, "八进制(8)"), (10, "十进制(10)"), (16, "十六进制(16)")] {
                        if ui.selectable_label(self.radix == r, name).clicked() { self.radix = r; self.compute(); }
                    }
                });

            let radix_for_layouter = self.radix;
            let mut layouter_fn = move |ui: &egui::Ui, text: &dyn egui::TextBuffer, wrap_width: f32| {
                let job = build_layout_job(text.as_str(), radix_for_layouter, wrap_width);
                ui.fonts(|f| f.layout_job(job))
            };
            let te = TextEdit::singleline(&mut self.input)
                .hint_text("在所选进制下输入表达式，如: A + B*10 或 1010 + 1111")
                .desired_width(360.0)
                .layouter(&mut layouter_fn);
            let resp = ui.add(te);

            if resp.changed() {
                // 实时计算（输入完成自动计算）
                self.compute();
            }

        });

        ui.add_space(8.0);

        if let Some(err) = &self.last_error {
            ui.colored_label(Color32::RED, RichText::new(err));
        }

        // 同时显示 2/8/10/16 进制的结果
        if self.last_error.is_none() {
            if let Some(val) = self.last_value {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("结果:").color(Color32::DARK_GREEN));
                        ui.monospace("");
                    });
                    for (r, label) in [(2u32, "二进制(2)"), (8, "八进制(8)"), (10, "十进制(10)"), (16, "十六进制(16)")] {
                        let s = format_auto(val, r, FRACTION_DIGITS);
                        ui.horizontal(|ui| {
                            ui.label(label);
                            ui.monospace(s);
                        });
                    }
                });
            }
        }

        ui.add_space(6.0);

        ui.separator();
        ui.collapsing("历史记录", |ui| {
            ui.horizontal(|ui| {
                if ui.button("清空历史").clicked() { self.history.clear(); }
            });
            for (idx, entry) in self.history.iter().rev().enumerate() {
                ui.horizontal_wrapped(|ui| {
                    let tag = format!("[{}]", entry.radix);
                    ui.monospace(tag);
                    ui.label(" ");
                    ui.label(RichText::new(&entry.input));
                    ui.label(" => ");
                    match &entry.error {
                        Some(err) => { ui.colored_label(Color32::RED, err); }
                        None => { ui.monospace(&entry.output); }
                    }
                    if !entry.decimal_expr.is_empty() {
                        ui.label(egui::RichText::new(format!("  (十进制: {})", entry.decimal_expr)).color(Color32::GRAY));
                    }
                    if ui.small_button("重用").clicked() {
                        self.radix = entry.radix;
                        self.input = entry.input.clone();
                        self.output.clear();
                        self.last_error = None;
                    }
                });
                if idx > 50 { break; } // 限制显示数量
            }
        });

        ui.separator();
        ui.collapsing("说明", |ui| {
            ui.label("• 表达式支持 + - * / % ^ 和括号 ()，以及函数名/常量（如 sin、cos、pi）");
            ui.label("• 在所选进制下输入数字，程序会在计算前自动转换为十进制交给 mathcore 计算");
            ui.label("• 计算后会将结果转换回所选进制显示（支持小数，保留符号）");
        });
    }

    fn compute(&mut self) {
        self.last_error = None;
        self.output.clear();
        self.last_value = None;

        let expr = self.input.trim();
        if expr.is_empty() { return; }
        let expr_owned = expr.to_string();

        match convert_expr_from_base(expr, self.radix) {
            Ok(decimal_expr) => {
                let math = MathCore::new();
                match math.calculate(&decimal_expr) {
                    Ok(val) => {
                        if !val.is_finite() {
                            self.last_error = Some("计算结果非有限数".to_string());
                            return;
                        }
                        self.last_value = Some(val);
                        self.output = format_auto(val, self.radix, FRACTION_DIGITS);
                        let out = self.output.clone();
                        self.push_history(&expr_owned, &decimal_expr, Some(out), None);
                    }
                    Err(e) => {
                        let msg = format!("计算失败: {e}");
                        self.last_error = Some(msg);
                        self.last_value = None;
                    }
                }
            }
            Err(e) => {
                self.last_error = Some(e);
                self.last_value = None;
            }
        }
    }
}

fn is_digit_in_radix(ch: char, radix: u32) -> bool {
    match ch {
        '0'..='9' => (ch as u32 - '0' as u32) < radix,
        'A'..='F' => (10 + (ch as u32 - 'A' as u32)) < radix,
        'a'..='f' => (10 + (ch as u32 - 'a' as u32)) < radix,
        '_' => true, // 允许下划线分隔符
        _ => false,
    }
}

fn convert_number_token(tok: &str, radix: u32) -> Result<i128, String> {
    let s = tok.replace('_', "");
    let neg = s.starts_with('-');
    let body = if neg { &s[1..] } else { &s[..] };
    if body.is_empty() { return Err("无效数字".to_string()); }
    if !body.chars().all(|c| is_digit_in_radix(c, radix)) { return Err(format!("包含超出基数 {radix} 的数字")); }
    let val = i128::from_str_radix(&body.to_uppercase(), radix).map_err(|_| "数字解析失败".to_string())?;
    Ok(if neg { -val } else { val })
}

/// 将表达式从指定进制转换为十进制表达式字符串
/// 支持：数字、标识符、+ - * / % ^ , 和括号；并支持隐式乘法（如 2(3+1), (1+2)(3+4), 2pi, pi2, x y, x(1+y) 等）
fn convert_expr_from_base(expr: &str, radix: u32) -> Result<String, String> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Kind { Start, Number, Ident, LParen, RParen, Op, Comma }


    let mut out = String::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0usize;

    let mut last_kind = Kind::Start;
    let mut last_ident: Option<String> = None;

    let is_op = |c: char| matches!(c, '+'|'-'|'*'|'/'|'%'|'^'|','|'(' | ')');

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() { i += 1; continue; }

        // 一元负号处理：基于 last_kind 判断
        let can_unary_minus = matches!(last_kind, Kind::Start | Kind::LParen | Kind::Op | Kind::Comma);
        if c == '-' && can_unary_minus {
            // 负号开头的数字
            let start = i;
            i += 1;
            let mut j = i;
            while j < chars.len() && is_digit_in_radix(chars[j], radix) { j += 1; }
            if j == i { return Err("一元负号后缺少数字".to_string()); }
            // 在开始处或可一元负号处，无需插入乘号
            let token: String = chars[start..j].iter().collect();
            let val = convert_number_token(&token, radix)?;
            out.push_str(&val.to_string());
            last_kind = Kind::Number;
            i = j;
            continue;
        }

        // 数字
        if is_digit_in_radix(c, radix) {
            let start = i;
            let mut j = i + 1;
            while j < chars.len() && is_digit_in_radix(chars[j], radix) { j += 1; }
            let token: String = chars[start..j].iter().collect();
            let val = convert_number_token(&token, radix)?;
            if matches!(last_kind, Kind::Number | Kind::RParen | Kind::Ident) { out.push('*'); }
            out.push_str(&val.to_string());
            last_kind = Kind::Number;
            i = j;
            continue;
        }

        // 括号与运算符
        if is_op(c) {
            match c {
                '(' => {
                    let insert = match last_kind {
                        Kind::Number | Kind::RParen => true,
                        Kind::Ident => {
                            if let Some(ref name) = last_ident { !is_function_like(name) } else { true }
                        }
                        _ => false,
                    };
                    if insert { out.push('*'); }
                    out.push('(');
                    last_kind = Kind::LParen;
                }
                ')' => {
                    out.push(')');
                    last_kind = Kind::RParen;
                }
                ',' => {
                    out.push(',');
                    last_kind = Kind::Comma;
                }
                _ => {
                    out.push(c);
                    last_kind = Kind::Op;
                }
            }
            i += 1;
            continue;
        }

        // 标识符（函数、常量等），直接透传
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            let mut j = i + 1;
            while j < chars.len() {
                let cj = chars[j];
                if cj.is_ascii_alphanumeric() || cj == '_' { j += 1; } else { break; }
            }
            let token: String = chars[start..j].iter().collect();
            if matches!(last_kind, Kind::Number | Kind::RParen | Kind::Ident) { out.push('*'); }
            out.push_str(&token);
            last_kind = Kind::Ident;
            last_ident = Some(token);
            i = j;
            continue;
        }

        return Err(format!("不支持的字符: {}", c));
    }

    Ok(out)
}

fn format_value_in_radix(val: i128, radix: u32) -> String {
    let neg = val < 0;
    let u = if neg { (-val) as u128 } else { val as u128 };
    let s = match radix {
        10 => u.to_string(),
        2 => format_radix(u, 2),
        8 => format_radix(u, 8),
        16 => format_radix_hex(u),
        _ => u.to_string(),
    };
    if neg { format!("-{s}") } else { s }
}

fn format_radix(mut v: u128, radix: u32) -> String {
    if v == 0 { return "0".to_string(); }
    let mut buf = Vec::new();
    while v > 0 { let d = (v % radix as u128) as u32; buf.push(char::from(b'0' + (d as u8))); v /= radix as u128; }
    buf.iter().rev().collect()
}

fn format_radix_hex(mut v: u128) -> String {
    if v == 0 { return "0".to_string(); }
    let mut buf = Vec::new();
    while v > 0 {
        let d = (v % 16) as u8;
        buf.push(match d { 0..=9 => (b'0' + d) as char, _ => (b'A' + (d - 10)) as char });
        v /= 16;
    }
    buf.iter().rev().collect()
}

fn format_float_in_radix(val: f64, radix: u32, frac_digits: usize) -> String {
    if !val.is_finite() { return "NaN".to_string(); }
    if radix == 10 {
        return trim_float(format!("{:.12}", val));
    }

    let neg = val.is_sign_negative();
    let abs = val.abs();
    let int_part_f = abs.trunc();

    // 若整数部分过大无法表示，回退十进制
    if int_part_f > (u128::MAX as f64) {
        return format!("{} (十进制)", trim_float(format!("{:.12}", val)));
    }

    let int_u = int_part_f as u128;
    let mut int_str = match radix {
        2 => format_radix(int_u, 2),
        8 => format_radix(int_u, 8),
        16 => format_radix_hex(int_u),
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

fn format_auto(val: f64, radix: u32, frac_digits: usize) -> String {
    let nearest = val.round();
    let tol = f64::max(1e-12, 1e-12 * nearest.abs());
    if (val - nearest).abs() <= tol && nearest.abs() <= (i128::MAX as f64) {
        return format_value_in_radix(nearest as i128, radix);
    }
    format_float_in_radix(val, radix, frac_digits)
}

fn trim_float(mut s: String) -> String {
    // 去除多余的尾随0与小数点
    if s.contains('.') {
        while s.ends_with('0') { s.pop(); }
        if s.ends_with('.') { s.pop(); }
    }
    s
}

impl RadixCalculator {
    fn push_history(&mut self, input: &str, decimal_expr: &str, output: Option<String>, error: Option<String>) {
        self.history.push_back(HistoryEntry {
            radix: self.radix,
            input: input.to_string(),
            decimal_expr: decimal_expr.to_string(),
            output: output.unwrap_or_default(),
            error,
        });
        while self.history.len() > MAX_HISTORY { self.history.pop_front(); }
    }
}

fn build_layout_job(text: &str, radix: u32, wrap_width: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.wrap.max_width = wrap_width;

    let default_fmt = TextFormat { font_id: FontId::monospace(14.0), color: Color32::BLACK, ..Default::default() };

    let invalid_fmt = TextFormat { font_id: FontId::monospace(14.0), color: Color32::RED, ..Default::default() };

    // 简单按"是否非法字符"进行高亮：非法字符标红，其余使用默认样式
    let mut last_byte = 0usize;
    let mut last_invalid = false;
    let mut started = false;

    for (byte_idx, ch) in text.char_indices() {
        let is_invalid = !is_valid_input_char(ch, radix);
        if !started {
            last_invalid = is_invalid;
            started = true;
        }
        if is_invalid != last_invalid {
            let slice = &text[last_byte..byte_idx];
            if !slice.is_empty() {
                job.append(slice, 0.0, if last_invalid { invalid_fmt.clone() } else { default_fmt.clone() });
            }
            last_byte = byte_idx;
            last_invalid = is_invalid;
        }
    }

    // 追加最后一段
    let slice = &text[last_byte..];
    if !slice.is_empty() {
        job.append(slice, 0.0, if last_invalid { invalid_fmt } else { default_fmt });
    }

    job
}

fn is_valid_input_char(c: char, radix: u32) -> bool {
    if c.is_whitespace() { return true; }
    if is_digit_in_radix(c, radix) { return true; }
    match c {
        '+' | '-' | '*' | '/' | '%' | '(' | ')' | '^' | ',' => true,
        '_' => true,
        _ => c.is_ascii_alphabetic(),
    }
}



fn is_function_like(name: &str) -> bool {
    match name.to_ascii_lowercase().as_str() {
        "sin"|"cos"|"tan"|"asin"|"acos"|"atan"
        |"log"|"ln"|"sqrt"|"abs"|"floor"|"ceil"|"round"
        |"exp"|"pow"|"min"|"max" => true,
        _ => false,
    }
}
