use crate::data::*;
use eframe::egui;
use egui::*;

pub fn basef32_32(data: &mut Data, ui: &mut Ui) {
    data.set_data_error(DataError::Nice);
    let mut input_data : f32 = 0.0;
    ui.horizontal(|ui| {
        ui.label(RichText::from("输入f32数据").color(Color32::BLUE)).on_hover_text("可输入下划线做视觉分割");
        let text_edit = TextEdit::singleline(&mut data.input_data)
        .desired_width(400.0);
        ui.add(text_edit);

        //允许输入"_"做视觉区分
        let raw_data = data.ref_input_data().clone().replace("_", "");

        match raw_data.parse::<f32>() {
            Ok(number) => input_data = number,
            Err(_) => {
                if raw_data.is_empty() {
                    data.set_data_error(DataError::LenNull);
                }else {
                data.set_data_error(DataError::FormatError);
                }
            },
        }
    });
    ui.horizontal(|ui| {
        match data.get_data_error() {
            DataError::FormatError => ui.colored_label(Color32::RED, "请输入f32数据"),
            DataError::LenNull => ui.colored_label(Color32::RED, "请输入数值"),
            DataError::Nice => {
                    let number_data = input_data.to_bits();
                    let string_data = format!("{:08x}", number_data);
                    data.set_output_data(string_data);
                    ui.add(Label::new(RichText::new("16进制编码").color(Color32::BLUE)));
                    ui.monospace(data.get_output_data())
            }
            _ => ui.colored_label(Color32::RED, "请输入f32数据")
        }
    });
}
