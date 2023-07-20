use bevy_ecs::system::{Res, Resource};

use super::{
    client::{ClientError, TMinreqRequest},
    marker::Endpoint,
};

pub type RespondsInner<S> = std::sync::Arc<std::sync::RwLock<Vec<S>>>;

#[derive(Debug, Resource)]
/// stores specific responds
pub struct Responds<Q, S> {
    pub responds: RespondsInner<Result<S, ClientError>>,
    /// we dont care what value request has
    pub _request: core::marker::PhantomData<Q>,
}

pub trait TRespondsReceived {
    fn read_unwrap_is_empty(&self) -> bool;
}

impl<Q, S> TRespondsReceived for Endpoint<Q, S>
where
    for<'a> Q: 'a + Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: 'a + Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
    Self: TMinreqRequest,
{
    fn read_unwrap_is_empty(&self) -> bool {
        self.storage.responds.read().unwrap().is_empty()
    }
}

#[derive(Debug)]
/// errors received from endpoint
pub enum RespondError {
    BadRequest(String),
    Other,
}

///  converts minreq response and error to marker specific response type and client error
pub fn handle_response<S: for<'d> serde::Deserialize<'d>>(
    response: Result<minreq::Response, minreq::Error>,
) -> Result<S, ClientError> {
    match response {
        Ok(response) => match response.status_code {
            // request success
            (200..=299) => Ok(response.json::<S>().unwrap()),

            // request failed
            (400..=499) => {
                let message = response.as_str().unwrap();

                Err(ClientError::Respond(RespondError::BadRequest(
                    message.to_string(),
                )))
            }

            // server error
            (500..=599) => Err(ClientError::Respond(RespondError::Other)),

            _ => Err(ClientError::Respond(RespondError::Other)),
        },
        Err(error) => Err(ClientError::Connection(error)),
    }
}

pub fn response_received<T>() -> impl Fn(Res<T>) -> bool
where
    T: Resource + TRespondsReceived,
{
    move |res: Res<T>| !res.read_unwrap_is_empty()
}
