use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type GetPublicAgent = Endpoint<(), GetPublicAgent200Response>;

impl GetPublicAgent {
    pub fn set_request(&self, rates: Rates, agent_symbol: String) {
        self.push_request(
            rates,
            minreq::Method::Get,
            format!("agents/{}", agent_symbol),
            None,
            None,
            Authorization::Unnecessary,
        );
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPublicAgent200Response {
    #[serde(rename = "data")]
    pub data: space_traders::models::Agent,
}
