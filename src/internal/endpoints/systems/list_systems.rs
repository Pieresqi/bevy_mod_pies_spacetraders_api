use crate::{
    internal::{client::QueryConf, endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ListSystems = Endpoint<(), space_traders::models::GetSystems200Response>;

impl ListSystems {
    pub fn set_request(
        &self,
        rates: Rates,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.push_request(
            rates,
            minreq::Method::Get,
            "systems".into(),
            QueryConf { limit, page }.into(),
            None,
            Authorization::Required,
        );
    }
}
