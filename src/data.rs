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
        let reversed: String = self.output_data.chars().rev().collect();

        for (i, c) in reversed.chars().enumerate() {
            if i > 0 && i % 4 == 0 {
                result.push('_');
            }
            result.push(c);
        }

        result.chars().rev().collect()
    }

    pub fn get_data_error(&self) -> &DataError {
        &self.data_error
    }

    pub fn set_data_error(&mut self, data_error: DataError) {
        self.data_error = data_error;
    }
}
