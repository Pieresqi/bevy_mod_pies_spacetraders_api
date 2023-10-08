use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type FulfillContract =
    Endpoint<(), pies_openapi_spacetraders_api::models::FulfillContract200Response>;

impl FulfillContract {
    pub fn set_request(&self, rates: Rates, contract_id: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/contracts/{}/fulfill", contract_id),
            None,
            None,
            Authorization::Required,
        );
    }
}
