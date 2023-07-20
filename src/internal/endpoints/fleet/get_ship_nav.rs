use crate::internal::endpoint::Endpoint;

pub type GetShipNav = Endpoint<(), pies_openapi_spacetraders_api::models::GetShipNav200Response>;

impl GetShipNav {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!("my/ships/{}/nav", ship_symbol)),
            None,
            None,
            true,
        );
    }
}
