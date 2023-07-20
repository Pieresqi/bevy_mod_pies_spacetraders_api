use crate::internal::endpoint::Endpoint;

pub type GetShipCargo =
    Endpoint<(), pies_openapi_spacetraders_api::models::GetMyShipCargo200Response>;

impl GetShipCargo {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!("my/ships/{}/cargo", ship_symbol)),
            None,
            None,
            true,
        );
    }
}
