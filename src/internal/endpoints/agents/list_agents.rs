use crate::{
    internal::{client::QueryConf, endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ListAgents = Endpoint<(), space_traders::models::GetAgents200Response>;

impl ListAgents {
    pub fn set_request(
        &self,
        rates: Rates,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Unnecessary)
                .set_additional_path("agents")
                .set_query(QueryConf { limit, page }),
        );
    }
}
