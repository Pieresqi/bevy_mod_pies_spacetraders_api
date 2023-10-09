use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ExtractResources = Endpoint<
    space_traders::models::ExtractResourcesRequest,
    space_traders::models::ExtractResources201Response,
>;

impl ExtractResources {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::ExtractResourcesRequest,
        ship_symbol: String,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/ships/{}/extract", ship_symbol))
                .set_body(request),
        );
    }
}
