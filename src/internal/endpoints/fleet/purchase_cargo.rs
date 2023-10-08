use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type PurchaseCargo = Endpoint<
    pies_openapi_spacetraders_api::models::PurchaseCargoRequest,
    pies_openapi_spacetraders_api::models::PurchaseCargo201Response,
>;

impl PurchaseCargo {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::PurchaseCargoRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/purchase", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
