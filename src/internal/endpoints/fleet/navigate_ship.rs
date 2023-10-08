use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
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
