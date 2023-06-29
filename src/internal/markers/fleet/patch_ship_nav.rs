use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type PatchShipNav =
    Marker<openapi::models::PatchShipNavRequest, openapi::models::GetShipNav200Response>;

impl TMinreqRequest for PatchShipNav {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Patch)
            .with_path(&format!("my/ships/{}/nav", args[0]))
            .with_body(body)
            .needs_bearer()
            .build()
    }
}

impl PatchShipNav {
    pub fn set_request(
        &mut self,
        request: openapi::models::PatchShipNavRequest,
        ship_symbol: String,
    ) {
        self.add_arg(ship_symbol);
        self.push_request(request);
    }
}
