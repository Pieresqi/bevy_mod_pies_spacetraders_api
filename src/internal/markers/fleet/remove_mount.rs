use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type RemoveMount = Marker<
    pies_openapi_spacetraders_api::models::RemoveMountRequest,
    pies_openapi_spacetraders_api::models::RemoveMount201Response,
>;

impl TMinreqRequest for RemoveMount {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/mounts/remove", args[0]))
            .needs_bearer()
            .with_body(body)
            .build()
    }
}

impl RemoveMount {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::RemoveMountRequest,
        ship_symbol: String,
    ) {
        self.add_arg(ship_symbol);
        self.push_request(request);
    }
}
