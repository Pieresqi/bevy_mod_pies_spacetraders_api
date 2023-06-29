use bevy_ecs::system::Resource;

use super::{
    client::{ClientConnectionConfig, ClientError, QueryConf, TMinreqRequest},
    marker::Marker,
    rate_limiter::Rates,
    respond::{handle_response, RespondsInner},
};

#[derive(Debug)]
/// errors caused by bad data in request
pub enum RequestError {
    BearerPrivateTokenNotSet,
    Serde(serde_json::Error),
}

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
    pub args: Vec<String>,
}

pub trait TRequest: std::fmt::Debug {
    /// sends requests and stores responses
    fn send_and_receive(&mut self, connection_config: ClientConnectionConfig, args: Vec<String>);
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
    pub request: Q,
    /// not all endpoints support query (page, limit)
    pub query: Option<QueryConf>,
}

impl<Q, S> TRequest for Request<Q, S>
where
    Q: Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
    Marker<Q, S>: TMinreqRequest,
{
    /// sends requests and stores responses
    fn send_and_receive(&mut self, connection_config: ClientConnectionConfig, args: Vec<String>) {
        let min_req = Marker::<Q, S>::try_create_minreq_request(
            connection_config,
            &self.request,
            &self.query,
            args,
        );

        let respond = match min_req {
            Ok(request) => handle_response::<S>(request.send()),
            Err(error) => Err(error),
        };

        self.responds.write().unwrap().push(respond);
    }
}
