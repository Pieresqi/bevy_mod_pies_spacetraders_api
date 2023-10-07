use bevy_ecs::{system::Resource, world::FromWorld};

use super::{
    client::{ClientError, QueryConf},
    rate_limiter::Rates,
    request::{Authorization, ChannelRequestHolder, Request, RequestInstance},
};

#[derive(Debug, Resource)]
pub struct Endpoint<Q, S>
where
    Q: Send + Sync + serde::Serialize, // what will be sent as json
    for<'a> S: Send + Sync + serde::Deserialize<'a>, // what will be received as json
{
    rates: Option<Rates>,
    channel_endpoint_receiver: crossbeam_channel::Receiver<Result<S, ClientError>>,
    channel_endpoint_sender: crossbeam_channel::Sender<Result<S, ClientError>>,
    channel_request_sender: crossbeam_channel::Sender<RequestInstance>,
    _dummy: std::marker::PhantomData<Q>,
}

impl<Q, S> Endpoint<Q, S>
where
    for<'a> Q: 'a + Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: 'a + Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
{
    pub fn set_rates(&mut self, rates: Rates) {
        self.rates = Some(rates);
    }

    pub fn push_request(
        &mut self,
        method: minreq::Method,
        path: Option<&str>,
        query: Option<QueryConf>,
        request: Option<Q>,
        needs_token: Authorization,
    ) {
        let request_h = RequestInstance {
            rates: self.rates.take().unwrap_or_default(),
            data: Box::new(Request {
                request,
                query,
                method: method,
                path: path.map(|inner| inner.to_string()),
                needs_token,
                channel_endpoint_sender: self.channel_endpoint_sender.clone(),
            }),
        };

        self.channel_request_sender.try_send(request_h).unwrap();
    }

    pub fn get_receiver(&self) -> &crossbeam_channel::Receiver<Result<S, ClientError>> {
        &self.channel_endpoint_receiver
    }
}

impl<Q, S> FromWorld for Endpoint<Q, S>
where
    for<'a> Q: 'a + Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: 'a + Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
{
    fn from_world(world: &mut bevy_ecs::world::World) -> Self {
        let channel_request = world.resource::<ChannelRequestHolder>();
        let (sender, receiver) = crossbeam_channel::unbounded();

        Self {
            rates: None,
            channel_request_sender: channel_request.sender.clone(),
            channel_endpoint_sender: sender,
            channel_endpoint_receiver: receiver,
            _dummy: std::marker::PhantomData,
        }
    }
}
