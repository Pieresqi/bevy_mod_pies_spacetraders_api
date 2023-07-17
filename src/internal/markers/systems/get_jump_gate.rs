use crate::internal::marker::Marker;

pub type GetJumpGate = Marker<(), pies_openapi_spacetraders_api::models::GetJumpGate200Response>;

impl GetJumpGate {
    pub fn set_request(&mut self, system_symbol: String, waypoint_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!(
                "systems/{}/waypoints/{}/jump-gate",
                system_symbol, waypoint_symbol
            )),
            None,
            None,
            true,
        );
    }
}
