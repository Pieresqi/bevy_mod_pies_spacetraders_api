use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ExtractResourcesWithSurvey = Endpoint<
    pies_openapi_spacetraders_api::models::Survey,
    pies_openapi_spacetraders_api::models::ExtractResources201Response,
>;

impl ExtractResourcesWithSurvey {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::Survey,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/extract/survey", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
