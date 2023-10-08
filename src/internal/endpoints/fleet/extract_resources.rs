use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
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
