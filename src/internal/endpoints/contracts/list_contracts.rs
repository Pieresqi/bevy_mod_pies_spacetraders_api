use crate::internal::{client::QueryConf, endpoint::Endpoint, request::Authorization};

pub type ListContracts = Endpoint<(), pies_openapi_spacetraders_api::models::GetContracts200Response>;

impl ListContracts {
    pub fn set_request(
        &mut self,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.push_request(
            minreq::Method::Get,
            "my/contracts".into(),
            QueryConf { limit, page }.into(),
            None,
            Authorization::Required,
        );
    }
}
