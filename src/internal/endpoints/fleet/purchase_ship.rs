use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type PurchaseShip = Endpoint<
    space_traders::models::PurchaseShipRequest,
    space_traders::models::PurchaseShip201Response,
>;

impl PurchaseShip {
    pub fn set_request(&self, rates: Rates, request: space_traders::models::PurchaseShipRequest) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_body(request)
                .set_additional_path("my/ships"),
        );
    }
}
