use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type GetMarket = Endpoint<(), pies_openapi_spacetraders_api::models::GetMarket200Response>;

impl GetMarket {
    pub fn set_request(&mut self, system_symbol: String, waypoint_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!(
                "systems/{}/waypoints/{}/market",
                system_symbol, waypoint_symbol
            )),
            None,
            None,
            Authorization::Required,
        );
    }
}
