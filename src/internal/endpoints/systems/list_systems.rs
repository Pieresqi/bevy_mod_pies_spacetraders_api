use crate::{
    internal::{client::QueryConf, endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type ListSystems = Endpoint<(), space_traders::models::GetSystems200Response>;

impl ListSystems {
    pub fn set_request(
        &self,
        rates: Rates,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path("systems")
                .set_query(QueryConf { limit, page }),
        );
    }
}
