use bevy_ecs::{system::Resource, world::FromWorld};

use crate::prelude::ClientConnectionConfig;

use super::{
    client::{ClientError, QueryConf, TMinreqRequest},
    minreq_request_builder::MinreqRequestBuilder,
    rate_limiter::Rates,
    request::{Request, RequestHolder, RequestsNew, Authorization},
    respond::Responds,
};

#[derive(Debug, Resource)]
pub struct Endpoint<Q, S>
where
    Q: Send + Sync + serde::Serialize, // what will be sent as json
    for<'a> S: Send + Sync + serde::Deserialize<'a>, // what will be received as json
{
    pub target: RequestsNew,
    pub storage: Responds<Q, S>,

    pub rates: Option<Rates>,
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
        self.target.requests.lock().unwrap().push(RequestHolder {
            rates: self.rates.take().unwrap_or_default(),
            data: Box::new(Request {
                responds: self.storage.responds.clone(),
                request,
                query,
                method: method,
                path: path.map(|inner| inner.to_string()),
                needs_token,
            }),
        });
    }

    pub fn write_unwrap(&mut self) -> std::sync::RwLockWriteGuard<'_, Vec<Result<S, ClientError>>> {
        self.storage.responds.write().unwrap()
    }

    pub fn get_last_and_clear(&mut self) -> Option<Result<S, ClientError>> {
        let mut storage = self.write_unwrap();

        let Some(respnse) = storage.pop() else {
            return None;
        };

        storage.clear();

        Some(respnse)
    }
}

impl<Q, S> FromWorld for Endpoint<Q, S>
where
    for<'a> Q: 'a + Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: 'a + Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
{
    fn from_world(world: &mut bevy_ecs::world::World) -> Self {
        let new = world.resource::<RequestsNew>().clone();
        Self {
            target: new,
            storage: Responds {
                responds: std::sync::Arc::new(std::sync::RwLock::new(Vec::new())),
                _request: std::marker::PhantomData::<Q>,
            },
            rates: None,
        }
    }
}

impl<Q, S> TMinreqRequest for Endpoint<Q, S>
where
    for<'a> Q: 'a + Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: 'a + Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
{
    fn try_create_minreq_request<B: serde::Serialize>(
        config: ClientConnectionConfig,
        body: Option<B>,
        query: Option<QueryConf>,
        method: minreq::Method,
        path: Option<String>,
        token: Authorization,
    ) -> Result<minreq::Request, ClientError> {
        MinreqRequestBuilder::new(config.bearer_token, config.path, method)
            .with_path(path)
            .with_body(body)
            .with_query(query)
            .with_bearer(token)
            .build()
    }
}
