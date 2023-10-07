use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type JumpShip = Endpoint<
    pies_openapi_spacetraders_api::models::JumpShipRequest,
    pies_openapi_spacetraders_api::models::JumpShip200Response,
>;

impl JumpShip {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::JumpShipRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/jump", ship_symbol)),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
