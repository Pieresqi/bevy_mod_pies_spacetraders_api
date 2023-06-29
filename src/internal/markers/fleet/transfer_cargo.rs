use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type TransferCargo =
    Marker<openapi::models::TransferCargoRequest, openapi::models::TransferCargo200Response>;

impl TMinreqRequest for TransferCargo {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/transfer", args[0]))
            .needs_bearer()
            .with_body(body)
            .build()
    }
}

impl TransferCargo {
    pub fn set_request(
        &mut self,
        request: openapi::models::TransferCargoRequest,
        ship_symbol: String,
    ) {
        self.add_arg(ship_symbol);
        self.push_request(request);
    }
}
