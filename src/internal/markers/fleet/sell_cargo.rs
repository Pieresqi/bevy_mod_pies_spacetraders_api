use crate::internal::marker::Marker;

pub type SellCargo = Marker<
    pies_openapi_spacetraders_api::models::SellCargoRequest,
    pies_openapi_spacetraders_api::models::SellCargo201Response,
>;

impl SellCargo {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::SellCargoRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/sell", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}
