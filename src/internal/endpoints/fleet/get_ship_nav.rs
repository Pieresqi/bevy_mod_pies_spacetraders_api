use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetShipNav = Endpoint<(), space_traders::models::GetShipNav200Response>;

impl GetShipNav {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("my/ships/{}/nav", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
