use crate::internal::marker::Marker;

pub type ExtractResources = Marker<
    pies_openapi_spacetraders_api::models::ExtractResourcesRequest,
    pies_openapi_spacetraders_api::models::ExtractResources201Response,
>;

impl ExtractResources {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::ExtractResourcesRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/extract", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}
