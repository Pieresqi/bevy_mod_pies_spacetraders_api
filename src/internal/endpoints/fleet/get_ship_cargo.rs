use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetShipCargo = Endpoint<(), space_traders::models::GetMyShipCargo200Response>;

impl GetShipCargo {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path(format!("my/ships/{ship_symbol}/cargo")),
        );
    }
}
