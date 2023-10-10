use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type JettisonCargo =
    Endpoint<space_traders::models::JettisonRequest, space_traders::models::Jettison200Response>;

impl JettisonCargo {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::JettisonRequest,
        ship_symbol: String,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/ships/{ship_symbol}/jettison"))
                .set_body(request),
        );
    }
}
