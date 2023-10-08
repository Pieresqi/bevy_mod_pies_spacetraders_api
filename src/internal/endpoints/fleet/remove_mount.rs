use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
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
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/mounts/remove", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
