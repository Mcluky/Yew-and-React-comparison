use fake::{Dummy, Fake};
use fake::faker::company::en::*;
use fake::faker::internet::en::*;
use fake::faker::phone_number::en::*;
use crate::extensions::search_text_filterable_trait::SearchTextFilterable;

#[derive(Debug, Dummy, PartialEq, Clone)]
pub struct Company {
    #[dummy(faker = "CompanyName()")]
    pub name: String,

    #[dummy(faker = "CatchPhase()")]
    pub catch_phrase: String,

    #[dummy(faker = "Industry()")]
    pub industry: String,

    #[dummy(faker = "PhoneNumber()")]
    pub phone_number: String,

    #[dummy(faker = "FreeEmail()")]
    pub email: String,

    #[dummy(faker = "IPv4()")]
    pub ipv4: String,
}

impl SearchTextFilterable for Company {
    fn contains(&self, text: &str) -> bool {
        self.name.to_lowercase().contains(text)
            || self.catch_phrase.to_lowercase().contains(text)
            || self.industry.to_lowercase().contains(text)
            || self.phone_number.to_lowercase().contains(text)
            || self.email.to_lowercase().contains(text)
            || self.ipv4.to_lowercase().contains(text)
    }
}