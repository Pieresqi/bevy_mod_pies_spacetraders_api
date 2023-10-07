use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type JettisonCargo = Endpoint<
    pies_openapi_spacetraders_api::models::JettisonRequest,
    pies_openapi_spacetraders_api::models::Jettison200Response,
>;

impl JettisonCargo {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::JettisonRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            format!("my/ships/{}/jettison", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
