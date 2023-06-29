use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type GetMarket = Marker<(), pies_openapi_spacetraders_api::models::GetMarket200Response>;

impl TMinreqRequest for GetMarket {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path(&format!("systems/{}/waypoints/{}/market", args[0], args[1]))
            .needs_bearer()
            .build()
    }
}

impl GetMarket {
    pub fn set_request(&mut self, system_symbol: String, waypoint_symbol: String) {
        self.add_arg(system_symbol);
        self.add_arg(waypoint_symbol);
        self.push_request(());
    }
}
