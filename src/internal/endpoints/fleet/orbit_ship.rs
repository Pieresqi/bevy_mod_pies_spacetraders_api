use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{Rates, MinreqRequestBuilder},
};

pub type OrbitShip = Endpoint<(), space_traders::models::OrbitShip200Response>;

impl OrbitShip {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.send_request(rates, MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required).set_additional_path(format!("my/ships/{}/orbit", ship_symbol)));
    }
}
