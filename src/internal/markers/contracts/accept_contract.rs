use crate::internal::marker::Marker;

pub type AcceptContract =
    Marker<(), pies_openapi_spacetraders_api::models::AcceptContract200Response>;

impl AcceptContract {
    pub fn set_request(&mut self, contract_id: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/contracts/{}/accept", contract_id)),
            None,
            None,
            true,
        );
    }
}
