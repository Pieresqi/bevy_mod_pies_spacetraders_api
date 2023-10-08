use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type PurchaseShip = Endpoint<
    space_traders::models::PurchaseShipRequest,
    space_traders::models::PurchaseShip201Response,
>;

impl PurchaseShip {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::PurchaseShipRequest,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            "my/ships".into(),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
