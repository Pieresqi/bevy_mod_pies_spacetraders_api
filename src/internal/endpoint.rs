use bevy_ecs::{system::Resource, world::FromWorld};

use super::{
    client::ClientError,
    minreq_request_builder::MinreqRequestBuilder,
    rate_limiter::Rates,
    request::{ChannelRequestHolder, RequestData, RequestDataInner, TRequest},
    respond::{TRespond, TRespondsReceived},
};

#[derive(Debug, Resource)]
pub struct Endpoint<Q, S>
where
    for<'a> Q: TRequest<'a>,
    for<'a> S: TRespond<'a>,
{
    channel_endpoint_receiver: crossbeam_channel::Receiver<Result<S, ClientError>>,
    channel_endpoint_sender: crossbeam_channel::Sender<Result<S, ClientError>>,
    channel_request_sender: crossbeam_channel::Sender<RequestData>,
    _dummy: std::marker::PhantomData<Q>,
}

impl<Q, S> Endpoint<Q, S>
where
    for<'a> Q: TRequest<'a> + std::fmt::Debug,
    for<'a> S: TRespond<'a> + std::fmt::Debug,
{
    pub fn send_request(&self, rates: Rates, builder: MinreqRequestBuilder<Q>) {
        let request_h = RequestData {
            rates,
            data: Box::new(RequestDataInner::new(
                self.channel_endpoint_sender.clone(),
                builder,
            )),
        };

        self.channel_request_sender.try_send(request_h).unwrap();
    }

    pub fn get_receiver(&self) -> &crossbeam_channel::Receiver<Result<S, ClientError>> {
        &self.channel_endpoint_receiver
    }
}

impl<Q, S> FromWorld for Endpoint<Q, S>
where
    for<'a> Q: TRequest<'a> + std::fmt::Debug,
    for<'a> S: TRespond<'a> + std::fmt::Debug,
{
    fn from_world(world: &mut bevy_ecs::world::World) -> Self {
        let channel_request = world.resource::<ChannelRequestHolder>();
        let (sender, receiver) = crossbeam_channel::unbounded();

        Self {
            channel_request_sender: channel_request.sender.clone(),
            channel_endpoint_sender: sender,
            channel_endpoint_receiver: receiver,
            _dummy: std::marker::PhantomData,
        }
    }
}

impl<Q, S> TRespondsReceived for Endpoint<Q, S>
where
    for<'a> Q: TRequest<'a> + std::fmt::Debug,
    for<'a> S: TRespond<'a> + std::fmt::Debug,
{
    fn receiver_is_empty(&self) -> bool {
        self.get_receiver().is_empty()
    }
}
