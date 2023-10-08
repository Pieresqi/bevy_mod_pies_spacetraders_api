use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type RegisterNewAgent = Endpoint<
    space_traders::models::RegisterRequest,
    space_traders::models::Register201Response,
>;

impl RegisterNewAgent {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::RegisterRequest,
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
