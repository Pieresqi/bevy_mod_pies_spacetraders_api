use crate::{
    internal::{client::QueryConf, endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type ListShips = Endpoint<(), pies_openapi_spacetraders_api::models::GetMyShips200Response>;

impl ListShips {
    pub fn set_request(
        &self,
        rates: Rates,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.push_request(
            rates,
            minreq::Method::Get,
            "my/ships".into(),
            QueryConf { limit, page }.into(),
            None,
            Authorization::Required,
        );
    }
}
