use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type CreateSurvey =
    Endpoint<(), space_traders::models::CreateSurvey201Response>;

impl CreateSurvey {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/survey", ship_symbol),
            None,
            None,
            Authorization::Required,
        )
    }
}
