use crate::{
    internal::{client::QueryConf, endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ListFactions = Endpoint<(), pies_openapi_spacetraders_api::models::GetFactions200Response>;

impl ListFactions {
    pub fn set_request(
        &self,
        rates: Rates,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.push_request(
            rates,
            minreq::Method::Get,
            "factions".into(),
            QueryConf { limit, page }.into(),
            None,
            Authorization::Required,
        );
    }
}
