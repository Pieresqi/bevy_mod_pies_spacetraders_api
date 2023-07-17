use crate::internal::marker::Marker;

pub type PurchaseCargo = Marker<
    pies_openapi_spacetraders_api::models::PurchaseCargoRequest,
    pies_openapi_spacetraders_api::models::PurchaseCargo201Response,
>;

impl PurchaseCargo {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::PurchaseCargoRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/purchase", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}
