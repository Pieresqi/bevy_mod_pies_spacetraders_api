use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type RegisterNewAgent =
    Endpoint<space_traders::models::RegisterRequest, space_traders::models::Register201Response>;

impl RegisterNewAgent {
    pub fn set_request(&self, rates: Rates, request: space_traders::models::RegisterRequest) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Unnecessary)
                .set_additional_path("register")
                .set_body(request),
        );
    }
}
