use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetWaypoint = Endpoint<(), space_traders::models::GetWaypoint200Response>;

impl GetWaypoint {
    pub fn set_request(&self, rates: Rates, system_symbol: String, waypoint_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("systems/{}/waypoints/{}", system_symbol, waypoint_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
