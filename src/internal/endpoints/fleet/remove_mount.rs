use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{Rates, MinreqRequestBuilder},
};

pub type RemoveMount = Endpoint<
    space_traders::models::RemoveMountRequest,
    space_traders::models::RemoveMount201Response,
>;

impl RemoveMount {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::RemoveMountRequest,
        ship_symbol: String,
    ) {
        self.send_request(rates, MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required).set_body(request).set_additional_path(format!("my/ships/{}/mounts/remove", ship_symbol)));
    }
}
