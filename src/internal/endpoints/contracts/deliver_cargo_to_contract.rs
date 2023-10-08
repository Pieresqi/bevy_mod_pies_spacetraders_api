use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type DeliverCargoToContract = Endpoint<
    pies_openapi_spacetraders_api::models::DeliverContractRequest,
    pies_openapi_spacetraders_api::models::DeliverContract200Response,
>;

impl DeliverCargoToContract {
    pub fn set_request(
        &self,
        rates: Rates,
        request: pies_openapi_spacetraders_api::models::DeliverContractRequest,
        contract_id: String,
    ) {
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/contracts/{}/deliver", contract_id),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
