use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type PurchaseShip = Endpoint<
    pies_openapi_spacetraders_api::models::PurchaseShipRequest,
    pies_openapi_spacetraders_api::models::PurchaseShip201Response,
>;

impl PurchaseShip {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::PurchaseShipRequest,
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
