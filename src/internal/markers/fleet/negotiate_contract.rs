use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type NegotiateContract = Marker<(), openapi::models::NegotiateContract200Response>;

impl TMinreqRequest for NegotiateContract {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Post)
            .with_path(&format!("my/ships/{}/negotiate/contract", args[0]))
            .needs_bearer()
            .build()
    }
}

impl NegotiateContract {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.add_arg(ship_symbol);
        self.push_request(());
    }
}
