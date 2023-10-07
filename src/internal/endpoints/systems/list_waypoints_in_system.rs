use crate::internal::{client::QueryConf, endpoint::Endpoint, request::Authorization};

pub type ListWaypointsInSystem =
    Endpoint<(), pies_openapi_spacetraders_api::models::GetSystemWaypoints200Response>;

impl ListWaypointsInSystem {
    pub fn set_request(
        &mut self,
        system_symbol: String,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.push_request(
            minreq::Method::Get,
            format!("systems/{}/waypoints", system_symbol),
            QueryConf { limit, page }.into(),
            None,
            Authorization::Required,
        );
    }
}
