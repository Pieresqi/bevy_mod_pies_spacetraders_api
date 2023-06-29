use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type ListContracts = Marker<(), openapi::models::GetContracts200Response>;

impl TMinreqRequest for ListContracts {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        _: &B,
        query: &Option<QueryConf>,
        _: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .needs_bearer()
            .with_path("my/contracts")
            .with_query(query)
            .build()
    }
}

impl ListContracts {
    pub fn set_request(
        &mut self,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.add_query(limit, page);
        self.push_request(());
    }
}
