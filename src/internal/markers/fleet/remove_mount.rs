use crate::internal::marker::Marker;

pub type RemoveMount = Marker<
    pies_openapi_spacetraders_api::models::RemoveMountRequest,
    pies_openapi_spacetraders_api::models::RemoveMount201Response,
>;

impl RemoveMount {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::RemoveMountRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/mounts/remove", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}
