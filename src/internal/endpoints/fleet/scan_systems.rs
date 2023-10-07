use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type ScanSystems =
    Endpoint<(), pies_openapi_spacetraders_api::models::CreateShipSystemScan201Response>;

impl ScanSystems {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/scan/systems", ship_symbol)),
            None,
            None,
            Authorization::Required,
        );
    }
}
