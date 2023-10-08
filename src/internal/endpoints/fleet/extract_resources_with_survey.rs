use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ExtractResourcesWithSurvey = Endpoint<
    space_traders::models::Survey,
    space_traders::models::ExtractResources201Response,
>;

impl ExtractResourcesWithSurvey {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::Survey,
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
