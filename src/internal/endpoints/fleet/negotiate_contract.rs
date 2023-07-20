use crate::internal::endpoint::Endpoint;

pub type NegotiateContract =
    Endpoint<(), pies_openapi_spacetraders_api::models::NegotiateContract200Response>;

impl NegotiateContract {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/negotiate/contract", ship_symbol)),
            None,
            None,
            true,
        );
    }
}
