use crate::internal::marker::Marker;

pub type GetFaction = Marker<(), pies_openapi_spacetraders_api::models::GetFaction200Response>;

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
