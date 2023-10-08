use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
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
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/refine", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
