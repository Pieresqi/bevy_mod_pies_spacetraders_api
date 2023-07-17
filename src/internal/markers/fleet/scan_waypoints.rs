use crate::internal::marker::Marker;

pub type ScanWaypoints =
    Marker<(), pies_openapi_spacetraders_api::models::CreateShipWaypointScan201Response>;

impl ScanWaypoints {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/scan/waypoints", ship_symbol)),
            None,
            None,
            true,
        );
    }
}
