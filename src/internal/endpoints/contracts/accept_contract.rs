use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type AcceptContract = Endpoint<(), space_traders::models::AcceptContract200Response>;

impl AcceptContract {
    pub fn set_request(&self, rates: Rates, contract_id: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/contracts/{contract_id}/accept")),
        );
    }
}
