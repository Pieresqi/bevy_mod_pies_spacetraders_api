use crate::internal::marker::Endpoint;

pub type GetFaction = Endpoint<(), pies_openapi_spacetraders_api::models::GetFaction200Response>;

impl GetFaction {
    pub fn set_request(&mut self, faction_symbol: String) {
        self.push_request(
            minreq::Method::Get,
            Some(&format!("factions/{}", faction_symbol)),
            None,
            None,
            true,
        )
    }
}
