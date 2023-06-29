use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type ShipRefine = Marker<
    pies_openapi_spacetraders_api::models::ShipRefineRequest,
    pies_openapi_spacetraders_api::models::ShipRefine201Response,
>;

impl TMinreqRequest for ShipRefine {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/refine", args[0]))
            .needs_bearer()
            .with_body(body)
            .build()
    }
}

impl ShipRefine {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::ShipRefineRequest,
        ship_symbol: String,
    ) {
        self.add_arg(ship_symbol);
        self.push_request(request);
    }
}
