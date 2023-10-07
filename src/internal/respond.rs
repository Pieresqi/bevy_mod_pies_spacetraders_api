use bevy_ecs::system::{Res, Resource};

use super::{client::ClientError, endpoint::Endpoint};

pub trait TRespondsReceived {
    fn receiver_is_empty(&self) -> bool;
}

impl<Q, S> TRespondsReceived for Endpoint<Q, S>
where
    for<'a> Q: 'a + Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: 'a + Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
{
    fn receiver_is_empty(&self) -> bool {
        self.get_receiver().is_empty()
    }
}

#[derive(Debug)]
/// errors received from endpoint
pub enum RespondError {
    BadRequest(String),
    Other(String),
}

impl std::fmt::Display for RespondError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = format!(
            "{}",
            match self {
                Self::BadRequest(s) => format!("Server responded: {}", s),
                Self::Other(s) => format!("Server or other: {}", s),
            }
        );

        write!(f, "{}", display)
    }
}

impl std::error::Error for RespondError {}

///  converts minreq response and error to marker specific response type and client error
pub fn handle_response<S: for<'d> serde::Deserialize<'d>>(
    response: Result<minreq::Response, minreq::Error>,
) -> Result<S, ClientError> {
    match response {
        Ok(response) => match response.status_code {
            // request success
            (200..=299) => Ok(response.json::<S>().unwrap()),

            // request failed
            (400..=499) => Err(ClientError::Respond(RespondError::BadRequest(
                response.as_str().to_string(),
            ))),

            // server error
            (500..=599) => Err(ClientError::Respond(RespondError::Other(
                response.as_str().to_string(),
            ))),

            _ => Err(ClientError::Respond(RespondError::Other(
                response.as_str().to_string(),
            ))),
        },
        Err(error) => Err(ClientError::Connection(error)),
    }
}

pub fn response_received<T>() -> impl Fn(Res<T>) -> bool
where
    T: Resource + TRespondsReceived,
{
    move |res: Res<T>| !res.receiver_is_empty()
}

// I am sorry.
trait MyToString {
    fn to_string(self) -> String;
}

impl<O, E> MyToString for Result<O, E>
where
    O: ToString,
    E: std::error::Error,
{
    fn to_string(self) -> String {
        self.map_or_else(|e| e.to_string(), |o| o.to_string())
    }
}
