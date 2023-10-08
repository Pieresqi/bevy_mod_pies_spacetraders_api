use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type DockShip = Endpoint<(), space_traders::models::DockShip200Response>;

impl DockShip {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/dock", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
