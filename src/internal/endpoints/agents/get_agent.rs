use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetAgent = Endpoint<(), space_traders::models::GetMyAgent200Response>;

impl GetAgent {
    pub fn set_request(&self, rates: Rates) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path("my/agent"),
        );
    }
}
