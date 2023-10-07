use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type GetMounts = Endpoint<(), pies_openapi_spacetraders_api::models::GetMounts200Response>;

impl GetMounts {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            format!("my/ships/{}/mounts", ship_symbol),
            None,
            None,
            Authorization::Required,
        );
    }
}
