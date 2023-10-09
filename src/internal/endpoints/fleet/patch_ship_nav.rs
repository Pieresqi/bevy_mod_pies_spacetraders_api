use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{Rates, MinreqRequestBuilder},
};

pub type PatchShipNav = Endpoint<
    space_traders::models::PatchShipNavRequest,
    space_traders::models::GetShipNav200Response,
>;

impl PatchShipNav {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::PatchShipNavRequest,
        ship_symbol: String,
    ) {
        self.send_request(rates, MinreqRequestBuilder::new(minreq::Method::Patch, Authorization::Required).set_body(request).set_additional_path(format!("my/ships/{}/nav", ship_symbol)));
    }
}
