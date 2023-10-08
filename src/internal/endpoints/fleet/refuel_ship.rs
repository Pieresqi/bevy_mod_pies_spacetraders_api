use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type RefuelShip = Endpoint<
    pies_openapi_spacetraders_api::models::RefuelShipRequest,
    pies_openapi_spacetraders_api::models::RefuelShip200Response,
>;

impl RefuelShip {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::RefuelShipRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/scan/refuel", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
