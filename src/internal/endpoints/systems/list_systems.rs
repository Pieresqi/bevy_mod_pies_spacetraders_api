use crate::{
    internal::{client::QueryConfig, endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ListSystems = Endpoint<(), space_traders::models::GetSystems200Response>;

impl ListSystems {
    pub fn set_request(
        &self,
        rates: Rates,
        query_config: QueryConfig
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path("systems")
                .set_query(query_config),
        );
    }
}
