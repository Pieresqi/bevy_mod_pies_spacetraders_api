use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type FulfillContract = Endpoint<(), space_traders::models::FulfillContract200Response>;

impl FulfillContract {
    pub fn set_request(&self, rates: Rates, contract_id: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/contracts/{}/fulfill", contract_id)),
        );
    }
}
