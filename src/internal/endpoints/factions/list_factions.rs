use crate::{
    internal::{client::QueryConf, endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ListFactions = Endpoint<(), space_traders::models::GetFactions200Response>;

impl ListFactions {
    pub fn set_request(
        &self,
        rates: Rates,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path("factions")
                .set_query(QueryConf { limit, page }),
        );
    }
}
