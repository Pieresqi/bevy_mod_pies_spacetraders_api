use crate::{
    internal::{client::QueryConf, endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ListAgents = Endpoint<(), space_traders::models::GetAgents200Response>;

impl ListAgents {
    pub fn set_request(
        &self,
        rates: Rates,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.push_request(
            rates,
            minreq::Method::Get,
            "agents".into(),
            QueryConf { limit, page }.into(),
            None,
            Authorization::Unnecessary,
        );
    }
}
