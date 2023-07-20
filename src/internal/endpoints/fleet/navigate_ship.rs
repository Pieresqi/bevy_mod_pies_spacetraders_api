use crate::internal::marker::Endpoint;

pub type NavigateShip = Endpoint<
    pies_openapi_spacetraders_api::models::NavigateShipRequest,
    pies_openapi_spacetraders_api::models::NavigateShip200Response,
>;

impl NavigateShip {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::NavigateShipRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/navigate", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}
