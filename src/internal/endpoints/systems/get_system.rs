use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetSystem = Endpoint<(), space_traders::models::GetSystem200Response>;

impl GetSystem {
    pub fn set_request(&self, rates: Rates, system_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path(format!("systems/{}", system_symbol)),
        );
    }
}
