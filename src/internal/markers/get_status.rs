use crate::internal::marker::Marker;

pub type GetStatus = Marker<(), pies_openapi_spacetraders_api::models::GetStatus200Response>;

impl GetStatus {
    pub fn set_request(&mut self) {
        self.push_request(minreq::Method::Get, None, None, None, false);
    }
}
