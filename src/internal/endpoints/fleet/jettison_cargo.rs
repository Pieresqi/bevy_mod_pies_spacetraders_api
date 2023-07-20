use crate::internal::marker::Endpoint;

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
            Some(&format!("my/ships/{}/jettison", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}
