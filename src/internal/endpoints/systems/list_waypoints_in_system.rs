use crate::{
    internal::{client::QueryConfig, endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ListWaypointsInSystem = Endpoint<(), space_traders::models::GetSystemWaypoints200Response>;

impl ListWaypointsInSystem {
    pub fn set_request(
        &self,
        rates: Rates,
        system_symbol: String,
        query_config: QueryConfig
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path(format!("systems/{system_symbol}/waypoints"))
                .set_query(query_config),
        );
    }
}
