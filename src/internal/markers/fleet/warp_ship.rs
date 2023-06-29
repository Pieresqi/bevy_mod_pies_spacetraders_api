use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type WarpShip = Marker<WarpShipRequest, openapi::models::NavigateShip200Response>;

impl TMinreqRequest for WarpShip {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/warp", args[0]))
            .needs_bearer()
            .with_body(body)
            .build()
    }
}

impl WarpShip {
    pub fn set_request(&mut self, request: WarpShipRequest, ship_symbol: String) {
        self.add_arg(ship_symbol);
        self.push_request(request);
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
