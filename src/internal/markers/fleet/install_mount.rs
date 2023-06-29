use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type InstallMount =
    Marker<openapi::models::InstallMountRequest, openapi::models::InstallMount201Response>;

impl TMinreqRequest for InstallMount {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/mounts/install", args[0]))
            .needs_bearer()
            .with_body(body)
            .build()
    }
}

impl InstallMount {
    pub fn set_request(
        &mut self,
        request: openapi::models::InstallMountRequest,
        ship_symbol: String,
    ) {
        self.add_arg(ship_symbol);
        self.push_request(request);
    }
}
