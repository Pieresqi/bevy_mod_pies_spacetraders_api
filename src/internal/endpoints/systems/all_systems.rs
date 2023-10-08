use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type AllSystems = Endpoint<(), Vec<pies_openapi_spacetraders_api::models::System>>;

impl AllSystems {
    pub fn set_request(&self, rates: Rates) {
        self.push_request(
            rates,
            minreq::Method::Get,
            "systems.json".into(),
            None,
            None,
            Authorization::Unnecessary,
        );
    }
}
