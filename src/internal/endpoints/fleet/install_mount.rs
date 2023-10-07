use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type InstallMount = Endpoint<
    pies_openapi_spacetraders_api::models::InstallMountRequest,
    pies_openapi_spacetraders_api::models::InstallMount201Response,
>;

impl InstallMount {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::InstallMountRequest,
        ship_symbol: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            format!("my/ships/{}/mounts/install", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
