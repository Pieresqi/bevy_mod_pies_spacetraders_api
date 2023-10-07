use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type SellCargo = Endpoint<
    pies_openapi_spacetraders_api::models::SellCargoRequest,
    pies_openapi_spacetraders_api::models::SellCargo201Response,
>;

impl SellCargo {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::SellCargoRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/sell", ship_symbol)),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
