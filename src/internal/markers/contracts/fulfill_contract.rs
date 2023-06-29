use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type FulfillContract = Marker<(), openapi::models::FulfillContract200Response>;

impl TMinreqRequest for FulfillContract {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/contracts/{}/fulfill", args[0]))
            .needs_bearer()
            .build()
    }
}

impl FulfillContract {
    pub fn set_request(&mut self, contract_id: String) {
        self.add_arg(contract_id);
        self.push_request(());
    }
}
