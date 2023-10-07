use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type GetContract = Endpoint<(), pies_openapi_spacetraders_api::models::GetContract200Response>;

impl GetContract {
    pub fn set_request(&mut self, contract_id: String) {
        self.push_request(
            minreq::Method::Get,
            format!("my/contracts/{}", contract_id),
            None,
            None,
            Authorization::Required,
        )
    }
}
