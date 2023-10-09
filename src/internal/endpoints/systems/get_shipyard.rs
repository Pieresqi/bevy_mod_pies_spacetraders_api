use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetShipyard = Endpoint<(), space_traders::models::GetShipyard200Response>;

impl GetShipyard {
    pub fn set_request(&self, rates: Rates, system_symbol: String, waypoint_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path(format!(
                    "systems/{}/waypoints/{}/shipyard",
                    system_symbol, waypoint_symbol
                )),
        );
    }
}
