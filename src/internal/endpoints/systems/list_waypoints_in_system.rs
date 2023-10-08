use crate::{
    internal::{client::QueryConf, endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ListWaypointsInSystem =
    Endpoint<(), pies_openapi_spacetraders_api::models::GetSystemWaypoints200Response>;

impl ListWaypointsInSystem {
    pub fn set_request(
        &self,
        rates: Rates,
        system_symbol: String,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("systems/{}/waypoints", system_symbol),
            QueryConf { limit, page }.into(),
            None,
            Authorization::Required,
        );
    }
}
