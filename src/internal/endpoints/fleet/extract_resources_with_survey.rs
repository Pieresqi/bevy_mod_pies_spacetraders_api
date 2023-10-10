use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ExtractResourcesWithSurvey =
    Endpoint<space_traders::models::Survey, space_traders::models::ExtractResources201Response>;

impl ExtractResourcesWithSurvey {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::Survey,
        ship_symbol: String,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/ships/{ship_symbol}/extract/survey"))
                .set_body(request),
        );
    }
}
