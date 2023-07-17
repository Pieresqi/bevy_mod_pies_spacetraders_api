use crate::internal::marker::Marker;

pub type PatchShipNav = Marker<
    pies_openapi_spacetraders_api::models::PatchShipNavRequest,
    pies_openapi_spacetraders_api::models::GetShipNav200Response,
>;

impl PatchShipNav {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::PatchShipNavRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Patch,
            Some(&format!("my/ships/{}/nav", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}
