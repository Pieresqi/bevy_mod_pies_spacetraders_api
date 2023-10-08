use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetFaction = Endpoint<(), pies_openapi_spacetraders_api::models::GetFaction200Response>;

impl GetFaction {
    pub fn set_request(&self, rates: Rates, faction_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("factions/{}", faction_symbol),
            None,
            None,
            Authorization::Required,
        )
    }
}
