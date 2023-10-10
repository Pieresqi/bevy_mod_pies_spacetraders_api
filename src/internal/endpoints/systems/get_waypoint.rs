use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetWaypoint = Endpoint<(), space_traders::models::GetWaypoint200Response>;

impl GetWaypoint {
    pub fn set_request(&self, rates: Rates, system_symbol: String, waypoint_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path(format!(
                    "systems/{system_symbol}/waypoints/{waypoint_symbol}"
                )),
        );
    }
}
