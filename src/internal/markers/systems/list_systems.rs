use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type ListSystems = Marker<(), openapi::models::GetSystems200Response>;

impl TMinreqRequest for ListSystems {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        _: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path("systems")
            .needs_bearer()
            .build()
    }
}

impl ListSystems {
    pub fn set_request(
        &mut self,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.add_query(limit, page);
        self.push_request(());
    }
}
