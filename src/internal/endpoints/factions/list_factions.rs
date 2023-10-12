use crate::{
    internal::{client::QueryConfig, endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ListFactions = Endpoint<(), space_traders::models::GetFactions200Response>;

impl ListFactions {
    pub fn set_request(
        &self,
        rates: Rates,
        query_config: QueryConfig
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path("factions")
                .set_query(query_config),
        );
    }
}
