use bevy_ecs::system::Resource;

use super::{
    client::{ClientConnectionConfig, ClientError},
    minreq_request_builder::MinreqRequestBuilder,
    rate_limiter::Rates,
    respond::{handle_response, TRespond},
};

pub trait TRequest: Send + Sync + serde::Serialize + 'static {}
impl<T> TRequest for T where T: Send + Sync + serde::Serialize + 'static {}

#[derive(Debug)]
pub enum Authorization {
    Required,
    Unnecessary,
}

pub trait TRequestDataInner {
    /// sends requests and stores responses
    fn send_and_receive(self: Box<Self>, connection_config: ClientConnectionConfig);
}

pub struct RequestData {
    pub rates: Rates,
    pub data: Box<dyn TRequestDataInner + Send + Sync>,
}

#[derive(Default, Resource)]
pub struct RequestsToBeProcessed {
    pub requests: Vec<RequestData>,
}

#[derive(Resource)]
pub struct ChannelRequestHolder {
    pub sender: crossbeam_channel::Sender<RequestData>,
    pub receiver: crossbeam_channel::Receiver<RequestData>,
}

impl Default for ChannelRequestHolder {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();

        Self { sender, receiver }
    }
}

pub struct RequestDataInner<Q, S>
where
    Q: TRequest,
    S: TRespond,
{
    channel_endpoint_sender: crossbeam_channel::Sender<Result<S, ClientError>>,
    builder: MinreqRequestBuilder<Q>,
}

impl<Q, S> RequestDataInner<Q, S>
where
    Q: TRequest,
    S: TRespond,
{
    pub fn new(
        channel_endpoint_sender: crossbeam_channel::Sender<Result<S, ClientError>>,
        builder: MinreqRequestBuilder<Q>,
    ) -> Self {
        Self {
            channel_endpoint_sender,
            builder,
        }
    }
}

impl<Q, S> TRequestDataInner for RequestDataInner<Q, S>
where
    Q: TRequest,
    S: TRespond,
{
    fn send_and_receive(self: Box<Self>, connection_config: ClientConnectionConfig) {
        let min_req = self
            .builder
            .build(connection_config.bearer_token, connection_config.base_path);

        let respond = handle_response::<S>(min_req.send());

        self.channel_endpoint_sender.try_send(respond).unwrap();
    }
}
