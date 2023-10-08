use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type JumpShip = Endpoint<
    space_traders::models::JumpShipRequest,
    space_traders::models::JumpShip200Response,
>;

impl JumpShip {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::JumpShipRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/jump", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
