use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type ListShips = Marker<(), openapi::models::GetMyShips200Response>;

impl TMinreqRequest for ListShips {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        _: &B,
        query: &Option<QueryConf>,
        _: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path("my/ships")
            .needs_bearer()
            .with_query(query)
            .build()
    }
}

impl ListShips {
    pub fn set_request(
        &mut self,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.add_query(limit, page);
        self.push_request(());
    }
}
