use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ScanWaypoints = Endpoint<(), space_traders::models::CreateShipWaypointScan201Response>;

impl ScanWaypoints {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/ships/{ship_symbol}/scan/waypoints")),
        );
    }
}
