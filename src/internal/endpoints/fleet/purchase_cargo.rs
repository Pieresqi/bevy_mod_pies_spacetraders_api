use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
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
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_body(request)
                .set_additional_path(format!("my/ships/{ship_symbol}/purchase")),
        );
    }
}
