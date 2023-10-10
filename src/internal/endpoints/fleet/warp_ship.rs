use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type WarpShip = Endpoint<WarpShipRequest, space_traders::models::NavigateShip200Response>;

impl WarpShip {
    pub fn set_request(&self, rates: Rates, request: WarpShipRequest, ship_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/ships/{ship_symbol}/warp"))
                .set_body(request),
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
