use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ExtractResources = Endpoint<
    pies_openapi_spacetraders_api::models::ExtractResourcesRequest,
    pies_openapi_spacetraders_api::models::ExtractResources201Response,
>;

impl ExtractResources {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::ExtractResourcesRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/extract", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
