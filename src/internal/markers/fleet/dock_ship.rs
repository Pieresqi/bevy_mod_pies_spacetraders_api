use crate::internal::marker::Marker;

pub type DockShip = Marker<(), pies_openapi_spacetraders_api::models::DockShip200Response>;

impl DockShip {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/dock", ship_symbol)),
            None,
            None,
            true,
        );
    }
}
