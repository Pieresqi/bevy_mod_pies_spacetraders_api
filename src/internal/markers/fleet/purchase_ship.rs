use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type PurchaseShip = Marker<
    pies_openapi_spacetraders_api::models::PurchaseShipRequest,
    pies_openapi_spacetraders_api::models::PurchaseShip201Response,
>;

impl TMinreqRequest for PurchaseShip {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        _: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path("my/ships")
            .needs_bearer()
            .with_body(body)
            .build()
    }
}

impl PurchaseShip {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::PurchaseShipRequest,
    ) {
        self.push_request(request);
    }
}
