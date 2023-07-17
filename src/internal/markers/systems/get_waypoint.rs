use crate::internal::marker::Marker;

pub type GetWaypoint = Marker<(), pies_openapi_spacetraders_api::models::GetWaypoint200Response>;

impl GetWaypoint {
    pub fn set_request(&mut self, system_symbol: String, waypoint_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!(
                "systems/{}/waypoints/{}",
                system_symbol, waypoint_symbol
            )),
            None,
            None,
            true,
        );
    }
}
