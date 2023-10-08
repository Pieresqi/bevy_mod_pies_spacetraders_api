use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetAgent = Endpoint<(), space_traders::models::GetMyAgent200Response>;

impl GetAgent {
    pub fn set_request(&self, rates: Rates) {
        self.push_request(
            rates,
            minreq::Method::Get,
            "my/agent".into(),
            None,
            None,
            Authorization::Required,
        );
    }
}
