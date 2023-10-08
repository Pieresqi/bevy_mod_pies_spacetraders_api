use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type CreateChart = Endpoint<(), pies_openapi_spacetraders_api::models::CreateChart201Response>;

impl CreateChart {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/chart", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
