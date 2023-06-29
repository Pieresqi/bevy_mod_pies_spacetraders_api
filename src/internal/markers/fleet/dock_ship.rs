use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type DockShip = Marker<(), pies_openapi_spacetraders_api::models::DockShip200Response>;

impl TMinreqRequest for DockShip {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/dock", args[0]))
            .needs_bearer()
            .build()
    }
}

impl DockShip {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.add_arg(ship_symbol);
        self.push_request(());
    }
}
