use crate::{
    internal::{client::QueryConfig, endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ListAgents = Endpoint<(), space_traders::models::GetAgents200Response>;

impl ListAgents {
    pub fn set_request(&self, rates: Rates, query_config: QueryConfig) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Unnecessary)
                .set_additional_path("agents")
                .set_query(query_config),
        );
    }
}
