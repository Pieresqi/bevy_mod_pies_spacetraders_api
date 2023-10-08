use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type OrbitShip = Endpoint<(), pies_openapi_spacetraders_api::models::OrbitShip200Response>;

impl OrbitShip {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/orbit", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
