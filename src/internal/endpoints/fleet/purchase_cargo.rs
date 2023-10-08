use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type PurchaseCargo = Endpoint<
    space_traders::models::PurchaseCargoRequest,
    space_traders::models::PurchaseCargo201Response,
>;

impl PurchaseCargo {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::PurchaseCargoRequest,
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
