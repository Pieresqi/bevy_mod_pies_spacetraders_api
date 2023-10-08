use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type RemoveMount = Endpoint<
    pies_openapi_spacetraders_api::models::RemoveMountRequest,
    pies_openapi_spacetraders_api::models::RemoveMount201Response,
>;

impl RemoveMount {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::RemoveMountRequest,
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
