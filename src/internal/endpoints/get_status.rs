use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetStatus = Endpoint<(), space_traders::models::GetStatus200Response>;

impl GetStatus {
    pub fn set_request(&self, rates: Rates) {
        self.push_request(
            rates,
            minreq::Method::Get,
            String::new(),
            None,
            None,
            Authorization::Unnecessary,
        );
    }
}
