use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type GetJumpGate = Endpoint<(), pies_openapi_spacetraders_api::models::GetJumpGate200Response>;

impl GetJumpGate {
    pub fn set_request(&mut self, system_symbol: String, waypoint_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            format!(
                "systems/{}/waypoints/{}/jump-gate",
                system_symbol, waypoint_symbol
            ),
            None,
            None,
            Authorization::Required,
        );
    }
}
