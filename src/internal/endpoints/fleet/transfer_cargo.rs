use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type TransferCargo = Endpoint<
    pies_openapi_spacetraders_api::models::TransferCargoRequest,
    pies_openapi_spacetraders_api::models::TransferCargo200Response,
>;

impl TransferCargo {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::TransferCargoRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/transfer", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
