use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type NavigateShip = Endpoint<
    pies_openapi_spacetraders_api::models::NavigateShipRequest,
    pies_openapi_spacetraders_api::models::NavigateShip200Response,
>;

impl NavigateShip {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::NavigateShipRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/navigate", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
