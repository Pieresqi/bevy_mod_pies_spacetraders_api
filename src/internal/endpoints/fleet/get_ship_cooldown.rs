use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type GetShipCooldown =
    Endpoint<(), pies_openapi_spacetraders_api::models::GetShipCooldown200Response>;

impl GetShipCooldown {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            format!("my/ships/{}/cooldown", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
