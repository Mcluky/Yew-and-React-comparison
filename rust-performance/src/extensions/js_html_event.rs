use yew::{Event, InputEvent};
use crate::extensions::js_html_input_field::InputField;

pub trait HtmlEvent {
    fn target_value(&self) -> String;
    fn target_number_value(&self, fallback: i32) -> i32;
}

impl HtmlEvent for Event {
    fn target_value(&self) -> String {
        self.target().unwrap().value_of().text_field_value()
    }

    fn target_number_value(&self, fallback: i32) -> i32 {
        self.target().unwrap().value_of().number_field_value(fallback)
    }
}

impl HtmlEvent for InputEvent {
    fn target_value(&self) -> String {
        self.target().unwrap().value_of().text_field_value()
    }

    fn target_number_value(&self, fallback: i32) -> i32 {
        self.target().unwrap().value_of().number_field_value(fallback)
    }
}
