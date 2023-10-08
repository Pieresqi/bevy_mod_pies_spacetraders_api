use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
};

pub type DeliverCargoToContract = Endpoint<
    space_traders::models::DeliverContractRequest,
    space_traders::models::DeliverContract200Response,
>;

impl DeliverCargoToContract {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::DeliverContractRequest,
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
