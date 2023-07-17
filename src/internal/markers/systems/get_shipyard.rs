use crate::internal::marker::Marker;

pub type GetShipyard = Marker<(), pies_openapi_spacetraders_api::models::GetShipyard200Response>;

impl GetShipyard {
    pub fn set_request(&mut self, system_symbol: String, waypoint_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!(
                "systems/{}/waypoints/{}/shipyard",
                system_symbol, waypoint_symbol
            )),
            None,
            None,
            true,
        );
    }
}
