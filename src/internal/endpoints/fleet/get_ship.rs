use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetShip = Endpoint<(), space_traders::models::GetMyShip200Response>;

impl GetShip {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("my/ships/{}", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
