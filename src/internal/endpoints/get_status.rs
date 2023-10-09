use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetStatus = Endpoint<(), space_traders::models::GetStatus200Response>;

impl GetStatus {
    pub fn set_request(&self, rates: Rates) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Unnecessary),
        );
    }
}
