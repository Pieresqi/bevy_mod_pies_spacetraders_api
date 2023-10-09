use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type GetPublicAgent = Endpoint<(), GetPublicAgent200Response>;

impl GetPublicAgent {
    pub fn set_request(&self, rates: Rates, agent_symbol: String) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Get, Authorization::Unnecessary)
                .set_additional_path(format!("agents/{}", agent_symbol)),
        );
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPublicAgent200Response {
    #[serde(rename = "data")]
    pub data: space_traders::models::Agent,
}
