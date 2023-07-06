use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type AllSystems = Marker<(), Vec<pies_openapi_spacetraders_api::models::System>>;

impl TMinreqRequest for AllSystems {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        _: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path("systems.json")
            .build()
    }
}

impl AllSystems {
    pub fn set_request(&mut self) {
        self.push_request(());
    }
}
