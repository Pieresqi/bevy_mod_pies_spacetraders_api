use crate::internal::endpoint::Endpoint;

pub type TransferCargo = Endpoint<
    pies_openapi_spacetraders_api::models::TransferCargoRequest,
    pies_openapi_spacetraders_api::models::TransferCargo200Response,
>;

impl TransferCargo {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::TransferCargoRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/transfer", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}
