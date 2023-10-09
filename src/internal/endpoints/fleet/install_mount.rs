use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::{MinreqRequestBuilder, Rates},
};

pub type InstallMount = Endpoint<
    space_traders::models::InstallMountRequest,
    space_traders::models::InstallMount201Response,
>;

impl InstallMount {
    pub fn set_request(
        &self,
        rates: Rates,
        request: space_traders::models::InstallMountRequest,
        ship_symbol: String,
    ) {
        self.send_request(
            rates,
            MinreqRequestBuilder::new(minreq::Method::Post, Authorization::Required)
                .set_body(request)
                .set_additional_path(format!("my/ships/{}/mounts/install", ship_symbol)),
        );
    }
}
