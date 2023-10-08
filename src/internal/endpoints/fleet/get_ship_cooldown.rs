use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetShipCooldown =
    Endpoint<(), pies_openapi_spacetraders_api::models::GetShipCooldown200Response>;

impl GetShipCooldown {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("my/ships/{}/cooldown", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
