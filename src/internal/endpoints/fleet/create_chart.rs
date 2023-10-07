use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type CreateChart = Endpoint<(), pies_openapi_spacetraders_api::models::CreateChart201Response>;

impl CreateChart {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/chart", ship_symbol)),
            None,
            None,
            Authorization::Required,
        );
    }
}
