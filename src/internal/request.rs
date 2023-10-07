use bevy_ecs::system::Resource;

use super::{
    client::{ClientConnectionConfig, ClientError, QueryConf},
    rate_limiter::Rates,
    respond::{handle_response, RespondsInner},
    minreq_request_builder::MinreqRequestBuilder
};

#[derive(Debug, Default, Resource, Clone)]
/// stores all new requests to be processed
pub struct RequestsNew {
    pub requests: std::sync::Arc<std::sync::Mutex<Vec<RequestHolder>>>,
}

#[derive(Debug, Default, Resource)]
/// stores all old requests to be sent before 'RequestsNew'
pub struct RequestsOld {
    pub requests: Vec<RequestHolder>,
}

#[derive(Debug)]
pub struct RequestHolder {
    pub rates: Rates,
    pub data: Box<dyn TRequest + Send + Sync>,
}

pub trait TRequest: std::fmt::Debug {
    /// sends requests and stores responses
    fn send_and_receive(self: Box<Self>, connection_config: ClientConnectionConfig);
}

#[derive(Debug)]
pub enum Authorization {
    Required,
    Unnecessary
}

#[derive(Debug)]
pub struct Request<Q, S>
where
    Q: Send + Sync + serde::Serialize,
    for<'a> S: Send + Sync + serde::Deserialize<'a>,
{
    /// put endpoint responses here
    pub responds: RespondsInner<Result<S, ClientError>>,
    /// actual data to be sent to the endpoint
    pub request: Option<Q>,
    /// not all endpoints support query (page, limit)
    pub query: Option<QueryConf>,
    pub method: minreq::Method,
    pub path: Option<String>,
    pub needs_token: Authorization,
}

impl<Q, S> TRequest for Request<Q, S>
where
    Q: Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
{
    /// sends requests and stores responses
    fn send_and_receive(self: Box<Self>, connection_config: ClientConnectionConfig) {

        let min_req = MinreqRequestBuilder::new(connection_config.bearer_token, connection_config.path, self.method)
            .with_path(self.path)
            .with_body(self.request)
            .with_query(self.query)
            .with_bearer(self.needs_token)
            .build();

        let respond = handle_response::<S>(min_req.send());

        self.responds.write().unwrap().push(respond);
    }
}
