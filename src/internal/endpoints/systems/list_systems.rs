use crate::internal::{client::QueryConf, endpoint::Endpoint, request::Authorization};

pub type ListSystems = Endpoint<(), pies_openapi_spacetraders_api::models::GetSystems200Response>;

impl ListSystems {
    pub fn set_request(
        &mut self,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.push_request(
            minreq::Method::Get,
            "systems".into(),
            QueryConf { limit, page }.into(),
            None,
            Authorization::Required,
        );
    }
}
