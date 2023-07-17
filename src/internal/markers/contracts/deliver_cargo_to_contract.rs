use crate::internal::marker::Marker;

pub type DeliverCargoToContract = Marker<
    pies_openapi_spacetraders_api::models::DeliverContractRequest,
    pies_openapi_spacetraders_api::models::DeliverContract200Response,
>;

impl DeliverCargoToContract {
    pub fn set_request(
        &mut self,
        request: pies_openapi_spacetraders_api::models::DeliverContractRequest,
        contract_id: String,
    ) {
        self.push_request(
            minreq::Method::Post,
            Some(&format!("my/contracts/{}/deliver", contract_id)),
            None,
            request.into(),
            true,
        );
    }
}
