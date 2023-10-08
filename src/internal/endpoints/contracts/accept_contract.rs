use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type AcceptContract =
    Endpoint<(), pies_openapi_spacetraders_api::models::AcceptContract200Response>;

impl AcceptContract {
    pub fn set_request(&self, rates: Rates, contract_id: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/contracts/{}/accept", contract_id),
            None,
            None,
            Authorization::Required,
        );
    }
}
