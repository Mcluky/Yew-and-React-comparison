use std::rc::Rc;
use std::time::Duration;
use fake::{Fake, Faker};
use log::info;
use yew::prelude::*;

use crate::components::company_table::CompanyTable;
use crate::components::control_panel::ControlPanel;
use crate::extensions::search_text_filterable_trait::SearchTextFilterable;
use crate::model::company::Company;
use crate::extensions::wasm_time::Instant;

pub struct App {
    company_data: Rc<Vec<Company>>,
    company_data_filtered: Rc<Vec<Company>>,
    search_text: String,
    amount_of_data: i32,
    last_required_time_for_search: Duration,
}

pub enum AppMessage {
    UpdateAmount { amount_of_data: i32 },
    GenerateData { amount_of_data: i32 },
    Search { search_text: String },
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            company_data: Rc::from(vec![]),
            company_data_filtered: Rc::from(vec![]),
            search_text: String::default(),
            amount_of_data: i32::default(),
            last_required_time_for_search: Duration::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            AppMessage::UpdateAmount { amount_of_data } => {
                self.amount_of_data = amount_of_data;
                true
            }
            AppMessage::GenerateData { amount_of_data } => {
                let mut new_company_data = vec![];
                for _i in 0..amount_of_data {
                    let new_company: Company = Faker::fake(&Faker);
                    new_company_data.push(new_company);
                }
                self.company_data = Rc::from(new_company_data);
                self.company_data_filtered = Rc::from(self.company_data.clone());
                self.search_text = String::default();
                true
            }
            AppMessage::Search { search_text } => {
                info!("Searching for text: {}", search_text);
                self.search_text = search_text;
                let case_insensitive_search_text = self.search_text.to_lowercase();

                self.company_data_filtered = Rc::from({
                    let now = Instant::now();
                    let filtered = self.company_data
                        .iter()
                        .filter(|company| company.contains(&case_insensitive_search_text))
                        .cloned()
                        .collect::<Vec<Company>>();
                    info!("Searching took: {:?}", now.elapsed());
                    self.last_required_time_for_search = now.elapsed();
                    filtered
                });
                true
            }
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        let on_submit = context.link()
            .callback(|amount_of_data| AppMessage::GenerateData { amount_of_data });
        let on_amount_of_data_changed = context.link()
            .callback(|amount_of_data| AppMessage::UpdateAmount { amount_of_data });
        let on_search_changes = context.link()
            .callback(|search_text: String| AppMessage::Search { search_text });

        html! {
            <main>
                <h1>{ "Hello, this is a test of yew, the rust framework 🦀!" }</h1>
                <ControlPanel amount_of_data={ self.amount_of_data }
                    on_amount_of_data_changed={ on_amount_of_data_changed }
                    on_submit={ on_submit }
                    on_search_changes={on_search_changes}
                    search_text={ self.search_text.to_owned() } />
                <p>{ format!("Required time for last search: {:?}",  self.last_required_time_for_search) }</p>
                <CompanyTable companies={ &self.company_data_filtered } />
            </main>
        }
    }
}