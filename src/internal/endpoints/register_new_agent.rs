use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type RegisterNewAgent = Endpoint<
    pies_openapi_spacetraders_api::models::RegisterRequest,
    pies_openapi_spacetraders_api::models::Register201Response,
>;

impl RegisterNewAgent {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::RegisterRequest,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            "register".into(),
            None,
            Some(request),
            Authorization::Unnecessary,
        );
    }
}
