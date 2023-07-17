use crate::internal::{client::QueryConf, marker::Marker};

pub type ListFactions = Marker<(), pies_openapi_spacetraders_api::models::GetFactions200Response>;

impl ListFactions {
    pub fn set_request(
        &mut self,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.push_request(
            minreq::Method::Get,
            "factions".into(),
            QueryConf { limit, page }.into(),
            None,
            true,
        );
    }
}
