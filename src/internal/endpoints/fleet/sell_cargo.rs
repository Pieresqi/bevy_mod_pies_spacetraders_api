use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type SellCargo = Endpoint<
    space_traders::models::SellCargoRequest,
    space_traders::models::SellCargo201Response,
>;

impl SellCargo {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::SellCargoRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/sell", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
