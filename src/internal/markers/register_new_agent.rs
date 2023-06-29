use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type RegisterNewAgent =
    Marker<pies_openapi_spacetraders_api::models::RegisterRequest, pies_openapi_spacetraders_api::models::Register201Response>;

impl TMinreqRequest for RegisterNewAgent {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        _: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path("register")
            .with_body(body)
            .build()
    }
}

impl RegisterNewAgent {
    pub fn set_request(&mut self, request: pies_openapi_spacetraders_api::models::RegisterRequest) {
        self.push_request(request);
    }
}
