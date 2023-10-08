use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ScanShips =
    Endpoint<(), space_traders::models::CreateShipShipScan201Response>;

impl ScanShips {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/scan/ships", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
