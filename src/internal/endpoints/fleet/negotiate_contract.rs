use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type NegotiateContract =
    Endpoint<(), pies_openapi_spacetraders_api::models::NegotiateContract200Response>;

impl NegotiateContract {
    pub fn set_request(&self, rates: Rates, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/negotiate/contract", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
