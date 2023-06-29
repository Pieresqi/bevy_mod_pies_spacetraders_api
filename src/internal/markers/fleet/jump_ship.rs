use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type JumpShip = Marker<openapi::models::JumpShipRequest, openapi::models::JumpShip200Response>;

impl TMinreqRequest for JumpShip {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/jump", args[0]))
            .with_body(body)
            .needs_bearer()
            .build()
    }
}

impl JumpShip {
    pub fn set_request(&mut self, request: openapi::models::JumpShipRequest, ship_symbol: String) {
        self.add_arg(ship_symbol);
        self.push_request(request);
    }
}
