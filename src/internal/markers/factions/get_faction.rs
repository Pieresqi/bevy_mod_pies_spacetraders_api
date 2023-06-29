use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type GetFaction = Marker<(), pies_openapi_spacetraders_api::models::GetFaction200Response>;

impl TMinreqRequest for GetFaction {
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        _: &B,
        _: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path(&format!("factions/{}", args[0]))
            .needs_bearer()
            .build()
    }
}

impl GetFaction {
    pub fn set_request(&mut self, faction_symbol: String) {
        self.add_arg(faction_symbol);
        self.push_request(());
    }
}
