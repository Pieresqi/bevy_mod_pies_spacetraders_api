use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type TransferCargo = Endpoint<
    space_traders::models::TransferCargoRequest,
    space_traders::models::TransferCargo200Response,
>;

impl TransferCargo {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::TransferCargoRequest,
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
