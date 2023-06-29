use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type NavigateShip = Marker<
    pies_openapi_spacetraders_api::models::NavigateShipRequest,
    pies_openapi_spacetraders_api::models::NavigateShip200Response,
>;

impl TMinreqRequest for NavigateShip {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/navigate", args[0]))
            .with_body(body)
            .needs_bearer()
            .build()
    }
}

impl NavigateShip {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::NavigateShipRequest,
        ship_symbol: String,
    ) {
        self.add_arg(ship_symbol);
        self.push_request(request);
    }
}
