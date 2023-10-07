use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type GetStatus = Endpoint<(), pies_openapi_spacetraders_api::models::GetStatus200Response>;

impl GetStatus {
    pub fn set_request(&mut self) {
        self.push_request(
            minreq::Method::Get,
            String::new(),
            None,
            None,
            Authorization::Unnecessary,
        );
    }
}
