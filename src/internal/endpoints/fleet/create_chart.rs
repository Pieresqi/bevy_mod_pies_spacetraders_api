use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type CreateChart = Endpoint<(), space_traders::models::CreateChart201Response>;

impl CreateChart {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/ships/{ship_symbol}/chart")),
        );
    }
}
