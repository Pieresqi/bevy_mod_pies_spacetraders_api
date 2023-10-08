use crate::{
    internal::{endpoint::Endpoint, request::Authorization},
    prelude::Rates,
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
        self.push_request(
            rates,
            minreq::Method::Post,
            format!("my/ships/{}/mounts/install", ship_symbol),
            None,
            request.into(),
            Authorization::Required,
        );
    }
}
