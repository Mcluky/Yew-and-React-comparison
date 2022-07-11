use yew::{Callback, html, InputEvent, prelude::function_component, Properties};

use crate::extensions::js_html_event::HtmlEvent;

#[derive(Properties, PartialEq)]
pub struct ControlPanelProperties {
    #[prop_or_default]
    pub amount_of_data: i32,
    pub on_amount_of_data_changed: Callback<i32>,
    pub on_submit: Callback<i32>,
    pub search_text: String,
    pub on_search_changes: Callback<String>,
}

#[function_component(ControlPanel)]
pub fn control_panel(props: &ControlPanelProperties) -> Html {
    let amount_of_data = props.amount_of_data;
    let search_text = props.search_text.to_owned();

    let parent_on_amount_of_data_changed = props.on_amount_of_data_changed.to_owned();
    let on_amount_input = Callback::from(move |event: InputEvent| parent_on_amount_of_data_changed.emit(event.target_number_value(amount_of_data)));

    let parent_on_submit = props.on_submit.to_owned();
    let on_submit_generate = Callback::from(move |_| parent_on_submit.emit(amount_of_data));

    let parent_on_search_text_change = props.on_search_changes.to_owned();
    let on_search_text_change = Callback::from(move |event: InputEvent| parent_on_search_text_change.emit(event.target_value()));

    html! {
            <div class="control-panel">
                <label for="amount_of_data">{"Amount of test data to generate: "}</label>
                <input type="text" id="amount_of_data" name="amount_of_data" value={amount_of_data.to_string()} oninput={on_amount_input} />
                <button class="button" onclick={on_submit_generate} >{"Generate"}</button>
                <br />
                <label for="search_txt">{"Search for text: "}</label>
                <input type="text" id="search_txt" name="search_txt" value={search_text} oninput={on_search_text_change} />
            </div>
    }
}