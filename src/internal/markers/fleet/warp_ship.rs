use crate::internal::marker::Marker;

pub type WarpShip =
    Marker<WarpShipRequest, pies_openapi_spacetraders_api::models::NavigateShip200Response>;

impl WarpShip {
    pub fn set_request(&mut self, request: WarpShipRequest, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/warp", ship_symbol)),
            None,
            request.into(),
            true,
        );
    }
}

#[derive(Clone, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
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
