use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetSystem = Endpoint<(), space_traders::models::GetSystem200Response>;

impl GetSystem {
    pub fn set_request(&self, rates: Rates, system_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("systems/{}", system_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
