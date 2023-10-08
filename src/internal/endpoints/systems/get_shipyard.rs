use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetShipyard = Endpoint<(), pies_openapi_spacetraders_api::models::GetShipyard200Response>;

impl GetShipyard {
    pub fn set_request(&self, rates: Rates, system_symbol: String, waypoint_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!(
                "systems/{}/waypoints/{}/shipyard",
                system_symbol, waypoint_symbol
            ),
            None,
            None,
            Authorization::Required,
        );
    }
}
