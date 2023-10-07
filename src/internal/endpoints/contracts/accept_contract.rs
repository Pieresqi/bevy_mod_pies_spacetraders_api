use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type AcceptContract =
    Endpoint<(), pies_openapi_spacetraders_api::models::AcceptContract200Response>;

impl AcceptContract {
    pub fn set_request(&mut self, contract_id: String) {
        self.push_request(
            minreq::Method::Post,
            format!("my/contracts/{}/accept", contract_id),
            None,
            None,
            Authorization::Required,
        );
    }
}
