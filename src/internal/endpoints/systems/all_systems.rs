use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type AllSystems = Endpoint<(), Vec<pies_openapi_spacetraders_api::models::System>>;

impl AllSystems {
    pub fn set_request(&mut self) {
        self.push_request(
            minreq::Method::Get,
            "systems.json".into(),
            None,
            None,
            Authorization::Unnecessary,
        );
    }
}
