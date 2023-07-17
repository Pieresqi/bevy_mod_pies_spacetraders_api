use crate::internal::marker::Marker;

pub type CreateChart = Marker<(), pies_openapi_spacetraders_api::models::CreateChart201Response>;

impl CreateChart {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/chart", ship_symbol)),
            None,
            None,
            true,
        );
    }
}
