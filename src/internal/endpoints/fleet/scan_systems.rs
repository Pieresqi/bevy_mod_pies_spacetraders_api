use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ScanSystems =
    Endpoint<(), space_traders::models::CreateShipSystemScan201Response>;

impl ScanSystems {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/scan/systems", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
