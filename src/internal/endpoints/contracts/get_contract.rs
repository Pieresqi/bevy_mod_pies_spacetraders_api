use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetContract = Endpoint<(), space_traders::models::GetContract200Response>;

impl GetContract {
    pub fn set_request(&self, rates: Rates, contract_id: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path(format!("my/contracts/{contract_id}")),
        );
    }
}
