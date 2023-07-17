use crate::internal::marker::Marker;

pub type GetMounts = Marker<(), pies_openapi_spacetraders_api::models::GetMounts200Response>;

impl GetMounts {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!("my/ships/{}/mounts", ship_symbol)),
            None,
            None,
            true,
        );
    }
}
