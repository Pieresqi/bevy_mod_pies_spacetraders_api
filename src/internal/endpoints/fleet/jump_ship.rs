use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type JumpShip =
    Endpoint<space_traders::models::JumpShipRequest, space_traders::models::JumpShip200Response>;

impl JumpShip {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::JumpShipRequest,
        ship_symbol: String,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/ships/{}/jump", ship_symbol))
                .set_body(request),
        );
    }
}
