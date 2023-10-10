use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetFaction = Endpoint<(), space_traders::models::GetFaction200Response>;

impl GetFaction {
    pub fn set_request(&self, rates: Rates, faction_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Required)
                .set_additional_path(format!("factions/{faction_symbol}")),
        );
    }
}
