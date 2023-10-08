use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type JettisonCargo = Endpoint<
    space_traders::models::JettisonRequest,
    space_traders::models::Jettison200Response,
>;

impl JettisonCargo {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::JettisonRequest,
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
