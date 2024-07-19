#[derive(PartialEq)]
pub enum DataError {
    FormatError,
    LenNull,
    LenOver,
    Nice,
}

pub struct Data {
    pub input_data: String,
    pub output_data: String,
    pub data_error: DataError,
}

impl Data {
    pub fn new() -> Data {
        Data {
            input_data: String::from(""),
            output_data: String::from(""),
            data_error: DataError::Nice,
        }
    }
    pub fn ref_input_data(&mut self) -> &mut String{
        &mut self.input_data
    }
    pub fn set_output_data(&mut self, output_data: String) {
        self.output_data = output_data;
    }
    pub fn get_output_data(&self) -> String {
        let mut result = String::new();
        let mut result_before_dot = String::new();
        if let Some(dot_pos) = self.output_data.find('.') {
            let (before_dot, after_dot) = self.output_data.split_at(dot_pos);
            //反转小数点前部分的字符串，用于插入下划线
            let reversed_before: String = before_dot.chars().rev().collect();
            for (i, c) in reversed_before.chars().enumerate() {
                if i > 0 && i % 4 == 0 {
                    result_before_dot.push('_');
                }
                result_before_dot.push(c);
            }
            //反转回来
            result_before_dot = result_before_dot.chars().rev().collect();
            let result_after_dot = after_dot.to_string();
            result = format!("{}{}", result_before_dot, result_after_dot);
        } else {
            //反转字符串，用于插入下划线
            let reversed: String = self.output_data.chars().rev().collect();
            for (i, c) in reversed.chars().enumerate() {
                if i > 0 && i % 4 == 0 {
                    result.push('_');
                }
                result.push(c);
            }
            //反转回来
            result = result.chars().rev().collect();
        }
        result
    }
    

    pub fn get_data_error(&self) -> &DataError {
        &self.data_error
    }

    pub fn set_data_error(&mut self, data_error: DataError) {
        self.data_error = data_error;
    }
}
