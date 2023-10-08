use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ShipRefine = Endpoint<
    pies_openapi_spacetraders_api::models::ShipRefineRequest,
    pies_openapi_spacetraders_api::models::ShipRefine201Response,
>;

impl ShipRefine {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::ShipRefineRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/refine", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
