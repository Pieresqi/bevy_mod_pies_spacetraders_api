use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetContract = Endpoint<(), pies_openapi_spacetraders_api::models::GetContract200Response>;

impl GetContract {
    pub fn set_request(&self, rates: Rates, contract_id: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("my/contracts/{}", contract_id),
            None,
            None,
            Authorization::Required,
        )
    }
}
