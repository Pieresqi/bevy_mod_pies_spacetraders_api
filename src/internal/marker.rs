use bevy_ecs::{system::Resource, world::FromWorld};

use super::{
    client::{ClientError, QueryConf, TMinreqRequest},
    rate_limiter::Rates,
    request::{Request, RequestHolder, RequestsNew},
    respond::Responds,
};

#[derive(Debug, Resource)]
pub struct Marker<Q, S>
where
    Q: Send + Sync + serde::Serialize, // what will be sent as json
    for<'a> S: Send + Sync + serde::Deserialize<'a>, // what will be received as json
    Self: TMinreqRequest,
{
    pub target: RequestsNew,
    pub storage: Responds<Q, S>,

    pub rates: Option<Rates>,
    // some endpoints require adding limit, page query params to endpoint
    pub query: Option<QueryConf>,
    // some endpoints require string arguments like contract_id, page....
    pub args: Vec<String>,
}

impl<Q, S> Marker<Q, S>
where
    for<'a> Q: 'a + Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: 'a + Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
    Self: TMinreqRequest,
{
    pub fn add_rates(&mut self, rates: Rates) {
        self.rates = Some(rates);
    }

    pub fn add_query(
        &mut self,
        limit: Option<core::num::NonZeroU8>,
        page: Option<core::num::NonZeroU8>,
    ) {
        self.query = Some(QueryConf { limit, page });
    }

    pub fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }

    pub fn push_request(&mut self, request: Q) {
        self.target.requests.lock().unwrap().push(RequestHolder {
            rates: self.rates.take().unwrap_or_default(),
            data: Box::new(Request {
                responds: self.storage.responds.clone(),
                request,
                query: self.query.take(),
            }),
            args: core::mem::take(&mut self.args),
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

impl<Q, S> FromWorld for Marker<Q, S>
where
    for<'a> Q: 'a + Send + Sync + serde::Serialize + std::fmt::Debug,
    for<'a> S: 'a + Send + Sync + serde::Deserialize<'a> + std::fmt::Debug,
    Marker<Q, S>: TMinreqRequest,
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
            query: None,
            args: Vec::new(),
        }
    }
}
