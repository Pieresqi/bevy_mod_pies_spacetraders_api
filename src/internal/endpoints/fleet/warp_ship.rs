use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type WarpShip = Endpoint<WarpShipRequest, space_traders::models::NavigateShip200Response>;

impl WarpShip {
    pub fn set_request(&self, rates: Rates, request: WarpShipRequest, ship_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/warp", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WarpShipRequest {
    /// The target destination.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
}

impl WarpShipRequest {
    pub fn new(waypoint_symbol: String) -> WarpShipRequest {
        WarpShipRequest { waypoint_symbol }
    }
}
