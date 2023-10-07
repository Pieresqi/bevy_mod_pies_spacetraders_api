use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type WarpShip =
    Endpoint<WarpShipRequest, pies_openapi_spacetraders_api::models::NavigateShip200Response>;

impl WarpShip {
    pub fn set_request(&mut self, request: WarpShipRequest, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            format!("my/ships/{}/warp", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
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
