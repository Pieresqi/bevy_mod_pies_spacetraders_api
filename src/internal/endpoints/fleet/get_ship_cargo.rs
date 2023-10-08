use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetShipCargo = Endpoint<(), space_traders::models::GetMyShipCargo200Response>;

impl GetShipCargo {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("my/ships/{}/cargo", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
