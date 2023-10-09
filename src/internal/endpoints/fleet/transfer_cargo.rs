use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
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
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_body(request)
                .set_additional_path(format!("my/ships/{}/transfer", ship_symbol)),
        );
    }
}
