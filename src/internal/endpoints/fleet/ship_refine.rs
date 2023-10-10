use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ShipRefine = Endpoint<
    space_traders::models::ShipRefineRequest,
    space_traders::models::ShipRefine201Response,
>;

impl ShipRefine {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::ShipRefineRequest,
        ship_symbol: String,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_body(request)
                .set_additional_path(format!("my/ships/{ship_symbol}/refine")),
        );
    }
}
