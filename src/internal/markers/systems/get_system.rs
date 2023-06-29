use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type GetSystem = Marker<(), pies_openapi_spacetraders_api::models::GetSystem200Response>;

impl TMinreqRequest for GetSystem {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path(&format!("systems/{}", args[0]))
            .needs_bearer()
            .build()
    }
}

impl GetSystem {
    pub fn set_request(&mut self, system_symbol: String) {
        self.add_arg(system_symbol);
        self.push_request(());
    }
}
