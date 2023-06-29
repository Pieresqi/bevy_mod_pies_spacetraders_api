use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type GetContract = Marker<(), openapi::models::GetContract200Response>;

impl TMinreqRequest for GetContract {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path(&format!("my/contracts/{}", args[0]))
            .needs_bearer()
            .build()
    }
}

impl GetContract {
    pub fn set_request(&mut self, contract_id: String) {
        self.add_arg(contract_id);
        self.push_request(());
    }
}
