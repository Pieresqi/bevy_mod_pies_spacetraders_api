use crate::internal::marker::Endpoint;

pub type GetShip = Endpoint<(), pies_openapi_spacetraders_api::models::GetMyShip200Response>;

impl GetShip {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!("my/ships/{}", ship_symbol)),
            None,
            None,
            true,
        );
    }
}
