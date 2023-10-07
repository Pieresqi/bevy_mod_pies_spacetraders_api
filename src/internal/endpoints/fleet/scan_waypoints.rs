use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type ScanWaypoints =
    Endpoint<(), pies_openapi_spacetraders_api::models::CreateShipWaypointScan201Response>;

impl ScanWaypoints {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            format!("my/ships/{}/scan/waypoints", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
