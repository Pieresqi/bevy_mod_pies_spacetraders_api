use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type GetStatus = Marker<(), pies_openapi_spacetraders_api::models::GetStatus200Response>;

impl TMinreqRequest for GetStatus {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        _: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            //.needs_bearer() // docs say it is needed but it works without it soooooo....
            .build()
    }
}

impl GetStatus {
    pub fn set_request(&mut self) {
        self.push_request(());
    }
}
