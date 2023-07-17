use crate::internal::marker::Marker;

pub type OrbitShip = Marker<(), pies_openapi_spacetraders_api::models::OrbitShip200Response>;

impl OrbitShip {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/orbit", ship_symbol)),
            None,
            None,
            true,
        );
    }
}
