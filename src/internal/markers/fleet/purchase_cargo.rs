use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type PurchaseCargo =
    Marker<openapi::models::PurchaseCargoRequest, openapi::models::PurchaseCargo201Response>;

impl TMinreqRequest for PurchaseCargo {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/purchase", args[0]))
            .needs_bearer()
            .with_body(body)
            .build()
    }
}

impl PurchaseCargo {
    pub fn set_request(
        &mut self,
        request: openapi::models::PurchaseCargoRequest,
        ship_symbol: String,
    ) {
        self.add_arg(ship_symbol);
        self.push_request(request);
    }
}
