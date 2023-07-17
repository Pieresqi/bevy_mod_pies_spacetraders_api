use crate::internal::marker::Marker;

pub type GetAgent = Marker<(), pies_openapi_spacetraders_api::models::GetMyAgent200Response>;

impl GetAgent {
    pub fn set_request(&mut self) {
        self.push_request(minreq::Method::Get, "my/agent".into(), None, None, true);
    }
}
