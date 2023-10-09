use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetShip = Endpoint<(), space_traders::models::GetMyShip200Response>;

impl GetShip {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path(format!("my/ships/{}", ship_symbol)),
        );
    }
}
