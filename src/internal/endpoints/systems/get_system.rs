use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type GetSystem = Endpoint<(), pies_openapi_spacetraders_api::models::GetSystem200Response>;

impl GetSystem {
    pub fn set_request(&mut self, system_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            format!("systems/{}", system_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
