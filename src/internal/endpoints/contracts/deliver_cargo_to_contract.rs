use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
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
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_additional_path(format!("my/contracts/{contract_id}/deliver"))
                .set_body(request),
        );
    }
}
