use log::error;
use wasm_bindgen::JsValue;

pub trait InputField {
    fn text_field_value(&self) -> String;
    fn number_field_value(&self, fallback: i32) -> i32;
}

impl InputField for js_sys::Object {
    fn text_field_value(&self) -> String {
        let value = js_sys::Reflect::get(&self, &JsValue::from_str("value")).unwrap();
        value.as_string().unwrap().to_string()
    }

    fn number_field_value(&self, fallback: i32) -> i32 {
        let mut value = self.text_field_value();
        if value.len() == 0 { // when field is empty assume 0
            value = "0".to_string();
        }
        let number = value.parse::<i32>();
        match number {
            Ok(number) => number,
            Err(_) => {
                error!("Error parsing number field value to number. Using fallback value: {}", fallback);
                fallback
            }
        }
    }
}