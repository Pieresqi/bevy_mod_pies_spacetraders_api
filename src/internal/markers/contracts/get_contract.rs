use crate::internal::marker::Marker;

pub type GetContract = Marker<(), pies_openapi_spacetraders_api::models::GetContract200Response>;

impl GetContract {
    pub fn set_request(&mut self, contract_id: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!("my/contracts/{}", contract_id)),
            None,
            None,
            true,
        )
    }
}
