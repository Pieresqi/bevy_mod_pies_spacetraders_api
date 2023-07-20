use crate::internal::marker::Endpoint;

pub type RegisterNewAgent = Endpoint<
    pies_openapi_spacetraders_api::models::RegisterRequest,
    pies_openapi_spacetraders_api::models::Register201Response,
>;

impl RegisterNewAgent {
    pub fn set_request(&mut self, request: pies_openapi_spacetraders_api::models::RegisterRequest) {
        self.push_request(
            minreq::Method::Post,
            "register".into(),
            None,
            Some(request),
            false,
        );
    }
}
