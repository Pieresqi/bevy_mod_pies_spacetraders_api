use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type ScanShips =
    Endpoint<(), pies_openapi_spacetraders_api::models::CreateShipShipScan201Response>;

impl ScanShips {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/scan/ships", ship_symbol)),
            None,
            None,
            Authorization::Required,
        );
    }
}
