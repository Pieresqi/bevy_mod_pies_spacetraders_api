use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetMarket = Endpoint<(), pies_openapi_spacetraders_api::models::GetMarket200Response>;

impl GetMarket {
    pub fn set_request(&self, rates: Rates, system_symbol: String, waypoint_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!(
                "systems/{}/waypoints/{}/market",
                system_symbol, waypoint_symbol
            ),
            None,
            None,
            Authorization::Required,
        );
    }
}
