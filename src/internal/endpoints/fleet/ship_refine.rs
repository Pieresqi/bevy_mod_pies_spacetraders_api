use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type ShipRefine = Endpoint<
    pies_openapi_spacetraders_api::models::ShipRefineRequest,
    pies_openapi_spacetraders_api::models::ShipRefine201Response,
>;

impl ShipRefine {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::ShipRefineRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            format!("my/ships/{}/refine", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
