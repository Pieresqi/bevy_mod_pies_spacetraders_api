use crate::internal::endpoint::Endpoint;

pub type PurchaseShip = Endpoint<
    pies_openapi_spacetraders_api::models::PurchaseShipRequest,
    pies_openapi_spacetraders_api::models::PurchaseShip201Response,
>;

impl PurchaseShip {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::PurchaseShipRequest,
    ) {
        self.push_request(
            minreq::Method::Post,
            "my/ships".into(),
            None,
            request.into(),
            true,
        );
    }
}
