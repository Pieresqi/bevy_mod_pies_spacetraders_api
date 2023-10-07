use crate::internal::{endpoint::Endpoint, request::Authorization};

pub type CreateSurvey =
    Endpoint<(), pies_openapi_spacetraders_api::models::CreateSurvey201Response>;

impl CreateSurvey {
    pub fn set_request(&mut self, ship_symbol: String) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/ships/{}/survey", ship_symbol)),
            None,
            None,
            Authorization::Required,
        )
    }
}
