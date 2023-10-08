use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type PatchShipNav = Endpoint<
    pies_openapi_spacetraders_api::models::PatchShipNavRequest,
    pies_openapi_spacetraders_api::models::GetShipNav200Response,
>;

impl PatchShipNav {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::PatchShipNavRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Patch,
            format!("my/ships/{}/nav", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
