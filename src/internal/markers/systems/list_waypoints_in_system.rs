use crate::internal::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
};

pub type ListWaypointsInSystem = Marker<(), pies_openapi_spacetraders_api::models::GetSystemWaypoints200Response>;

impl TMinreqRequest for ListWaypointsInSystem {
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        _: &B,
        query: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError> {
        config
            .new_builder::<B>()
            .with_method(minreq::Method::Get)
            .with_path(&format!("systems/{}/waypoints", args[0]))
            .needs_bearer()
            .with_query(query)
            .build()
    }
}

impl ListWaypointsInSystem {
    pub fn set_request(
        &mut self,
        system_symbol: String,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.add_arg(system_symbol);
        self.add_query(limit, page);
        self.push_request(());
    }
}
