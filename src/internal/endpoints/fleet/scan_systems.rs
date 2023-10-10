use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ScanSystems = Endpoint<(), space_traders::models::CreateShipSystemScan201Response>;

impl ScanSystems {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/ships/{ship_symbol}/scan/systems")),
        );
    }
}
