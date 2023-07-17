use crate::internal::marker::Marker;

pub type FulfillContract =
    Marker<(), pies_openapi_spacetraders_api::models::FulfillContract200Response>;

impl FulfillContract {
    pub fn set_request(&mut self, contract_id: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/contracts/{}/fulfill", contract_id)),
            None,
            None,
            true,
        );
    }
}
