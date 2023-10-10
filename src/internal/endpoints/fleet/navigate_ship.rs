use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type NavigateShip = Endpoint<
    space_traders::models::NavigateShipRequest,
    space_traders::models::NavigateShip200Response,
>;

impl NavigateShip {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::NavigateShipRequest,
        ship_symbol: String,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/ships/{ship_symbol}/navigate"))
                .set_body(request),
        );
    }
}
