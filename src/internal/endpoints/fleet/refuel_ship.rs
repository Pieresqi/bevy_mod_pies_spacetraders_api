use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{Rates, MinreqRequestBuilder},
};

pub type RefuelShip = Endpoint<
    space_traders::models::RefuelShipRequest,
    space_traders::models::RefuelShip200Response,
>;

impl RefuelShip {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::RefuelShipRequest,
        ship_symbol: String,
    ) {
        self.send_request(rates, MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required).set_body(request).set_additional_path(format!("my/ships/{}/scan/refuel", ship_symbol)));
    }
}
