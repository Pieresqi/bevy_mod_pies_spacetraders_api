use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type ExtractResources =
    Marker<openapi::models::ExtractResourcesRequest, openapi::models::ExtractResources201Response>;

impl TMinreqRequest for ExtractResources {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/extract", args[0]))
            .with_body(body)
            .needs_bearer()
            .build()
    }
}

impl ExtractResources {
    pub fn set_request(
        &mut self,
        request: openapi::models::ExtractResourcesRequest,
        ship_symbol: String,
    ) {
        self.add_arg(ship_symbol);
        self.push_request(request);
    }
}
