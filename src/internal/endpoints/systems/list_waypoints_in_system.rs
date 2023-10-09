use crate::{
    internal::{client::QueryConf, endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ListWaypointsInSystem = Endpoint<(), space_traders::models::GetSystemWaypoints200Response>;

impl ListWaypointsInSystem {
    pub fn set_request(
        &self,
        rates: Rates,
        system_symbol: String,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path(format!("systems/{}/waypoints", system_symbol))
                .set_query(QueryConf { limit, page }),
        );
    }
}
