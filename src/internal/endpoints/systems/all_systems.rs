use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type AllSystems = Endpoint<(), Vec<space_traders::models::System>>;

impl AllSystems {
    pub fn set_request(&self, rates: Rates) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Unnecessary)
                .set_additional_path("systems.json"),
        );
    }
}
