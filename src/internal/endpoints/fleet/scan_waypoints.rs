use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ScanWaypoints = Endpoint<(), space_traders::models::CreateShipWaypointScan201Response>;

impl ScanWaypoints {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/scan/waypoints", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
