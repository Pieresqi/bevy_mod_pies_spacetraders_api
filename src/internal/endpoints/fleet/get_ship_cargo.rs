use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type GetShipCargo =
    Endpoint<(), pies_openapi_spacetraders_api::models::GetMyShipCargo200Response>;

impl GetShipCargo {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            format!("my/ships/{}/cargo", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
