use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type JettisonCargo = Endpoint<
    pies_openapi_spacetraders_api::models::JettisonRequest,
    pies_openapi_spacetraders_api::models::Jettison200Response,
>;

impl JettisonCargo {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::JettisonRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/jettison", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
