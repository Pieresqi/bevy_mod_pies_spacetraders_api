use crate::internal::endpoint::Endpoint;

pub type GetSystem = Endpoint<(), pies_openapi_spacetraders_api::models::GetSystem200Response>;

impl GetSystem {
    pub fn set_request(&mut self, system_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!("systems/{}", system_symbol)),
            None,
            None,
            true,
        );
    }
}
