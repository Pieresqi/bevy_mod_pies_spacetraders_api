use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type DeliverCargoToContract = Marker<
    pies_openapi_spacetraders_api::models::DeliverContractRequest,
    pies_openapi_spacetraders_api::models::DeliverContract200Response,
>;

impl TMinreqRequest for DeliverCargoToContract {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/contracts/{}/deliver", args[0]))
            .with_body(body)
            .needs_bearer()
            .build()
    }
}

impl DeliverCargoToContract {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::DeliverContractRequest,
        contract_id: String,
    ) {
        self.add_arg(contract_id);
        self.push_request(request);
    }
}
