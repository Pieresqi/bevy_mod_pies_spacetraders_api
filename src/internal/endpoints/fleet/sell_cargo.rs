use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type SellCargo =
    Endpoint<space_traders::models::SellCargoRequest, space_traders::models::SellCargo201Response>;

impl SellCargo {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::SellCargoRequest,
        ship_symbol: String,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_body(request)
                .set_additional_path(format!("my/ships/{ship_symbol}/sell")),
        );
    }
}
