use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type GetAgent = Marker<(), pies_openapi_spacetraders_api::models::GetMyAgent200Response>;

impl TMinreqRequest for GetAgent {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        _: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path("my/agent")
            .needs_bearer()
            .build()
    }
}

impl GetAgent {
    pub fn set_request(&mut self) {
        self.push_request(());
    }
}
