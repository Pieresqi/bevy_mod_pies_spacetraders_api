use crate::internal::marker::Endpoint;

pub type RefuelShip = Endpoint<
    pies_openapi_spacetraders_api::models::RefuelShipRequest,
    pies_openapi_spacetraders_api::models::RefuelShip200Response,
>;

impl RefuelShip {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::RefuelShipRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/scan/refuel", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}
