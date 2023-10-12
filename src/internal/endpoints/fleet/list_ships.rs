use crate::{
    internal::{client::QueryConfig, endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ListShips = Endpoint<(), space_traders::models::GetMyShips200Response>;

impl ListShips {
    pub fn set_request(
        &self,
        rates: Rates,
        query_config: QueryConfig
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path("my/ships")
                .set_query(query_config),
        );
    }
}
