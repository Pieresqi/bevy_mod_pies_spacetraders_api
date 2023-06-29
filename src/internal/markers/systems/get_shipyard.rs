use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type GetShipyard = Marker<(), openapi::models::GetShipyard200Response>;

impl TMinreqRequest for GetShipyard {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path(&format!(
                "systems/{}/waypoints/{}/shipyard",
                args[0], args[1]
            ))
            .needs_bearer()
            .build()
    }
}

impl GetShipyard {
    pub fn set_request(&mut self, system_symbol: String, waypoint_symbol: String) {
        self.add_arg(system_symbol);
        self.add_arg(waypoint_symbol);
        self.push_request(());
    }
}
