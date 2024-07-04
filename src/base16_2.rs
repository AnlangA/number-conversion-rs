use crate::data::*;
use eframe::egui;
use egui::*;
use num::BigUint;

pub fn base16_2(data: &mut Data, ui: &mut Ui) {
    data.set_data_error(DataError::Nice);
    let mut input_data = String::new();
    ui.horizontal(|ui| {
        ui.label(RichText::from("16进制数").color(Color32::BLUE)).on_hover_text("可输入下划线做视觉分割");
        let text_edit = TextEdit::singleline(&mut data.input_data)
        .desired_width(400.0);
        ui.add(text_edit);

        //允许输入"_"做视觉区分
        let raw_data = data.ref_input_data().clone().replace("_", "");

        if raw_data.is_empty() {
            data.set_data_error(DataError::LenNull);
        }else if raw_data.len() > 16 {
            data.set_data_error(DataError::LenOver);
        }
        
        input_data = raw_data
            .chars()
            .filter(|c| {
                if !c.is_digit(16) {
                    data.set_data_error(DataError::FormatError);
                    false
                } else {
                    true
                }
            })
            .collect();
    });
    ui.horizontal(|ui| {
        match data.get_data_error() {
            DataError::FormatError => ui.colored_label(Color32::RED, "请输入16进制字符"),
            DataError::LenNull => ui.colored_label(Color32::RED, "请输入数值"),
            DataError::LenOver => ui.colored_label(Color32::RED, "数值长度超过16位"),
            DataError::Nice => {
                    let number_data = u64::from_str_radix(&input_data, 16).unwrap();
                    let string_data = BigUint::from(number_data).to_str_radix(2);
                    data.set_output_data(string_data);
                    ui.add(Label::new(RichText::new("2进制数").color(Color32::BLUE)));
                    ui.monospace(data.get_output_data())
            }
        }
    });
}
