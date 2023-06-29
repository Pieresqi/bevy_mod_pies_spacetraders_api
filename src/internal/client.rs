use bevy_app::{App, CoreSet, Plugin};
use bevy_ecs::{
    schedule::IntoSystemConfig,
    system::{Res, ResMut, Resource},
};
use bevy_tasks::IoTaskPool;
use bevy_time::common_conditions::on_timer;

use super::{
    markers::{
        agents::get_agent::GetAgent,
        contracts::{
            accept_contract::AcceptContract, deliver_cargo_to_contract::DeliverCargoToContract,
            fulfill_contract::FulfillContract, get_contract::GetContract,
            list_contracts::ListContracts,
        },
        factions::{get_faction::GetFaction, list_factions::ListFactions},
        fleet::{
            create_chart::CreateChart, create_survey::CreateSurvey, dock_ship::DockShip,
            extract_resources::ExtractResources, get_mounts::GetMounts, get_ship::GetShip,
            get_ship_cargo::GetShipCargo, get_ship_cooldown::GetShipCooldown,
            get_ship_nav::GetShipNav, install_mount::InstallMount, jettison_cargo::JettisonCargo,
            jump_ship::JumpShip, list_ships::ListShips, navigate_ship::NavigateShip,
            negotiate_contract::NegotiateContract, orbit_ship::OrbitShip,
            patch_ship_nav::PatchShipNav, purchase_cargo::PurchaseCargo,
            purchase_ship::PurchaseShip, refuel_ship::RefuelShip, remove_mount::RemoveMount,
            scan_ships::ScanShips, scan_systems::ScanSystems, scan_waypoints::ScanWaypoints,
            sell_cargo::SellCargo, ship_refine::ShipRefine, transfer_cargo::TransferCargo,
            warp_ship::WarpShip,
        },
        get_status::GetStatus,
        register_new_agent::RegisterNewAgent,
        systems::{
            get_jump_gate::GetJumpGate, get_market::GetMarket, get_shipyard::GetShipyard,
            get_system::GetSystem, get_waypoint::GetWaypoint, list_systems::ListSystems,
            list_waypoints_in_system::ListWaypointsInSystem,
        },
    },
    minreq_request_builder::MinreqRequestBuilderUnready,
    rate_limiter::{replenish_buckets_step, RateBucket, RateLimit, RateStrategy, RS},
    request::{RequestError, RequestsNew, RequestsOld},
    respond::RespondError,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(dispatch_requests.in_base_set(CoreSet::PostUpdate).run_if(
            move |new: Res<RequestsNew>, old: Res<RequestsOld>, bucket: Res<RateBucket>| {
                (!new.requests.lock().unwrap().is_empty() || !old.requests.is_empty())
                    && (bucket.inner.peek(RateLimit::Normal) || bucket.inner.peek(RateLimit::Burst))
            },
        ))
        .add_system(replenish_buckets_step.run_if(on_timer(std::time::Duration::from_secs_f32(RS))))
        .init_resource::<RequestsNew>()
        .init_resource::<RequestsOld>()
        .init_resource::<RateBucket>()
        .init_resource::<ClientConnectionConfig>()
        .init_resource::<GetStatus>()
        .init_resource::<RegisterNewAgent>()
        // agents
        .init_resource::<GetAgent>()
        // contracts
        .init_resource::<AcceptContract>()
        .init_resource::<DeliverCargoToContract>()
        .init_resource::<FulfillContract>()
        .init_resource::<GetContract>()
        .init_resource::<ListContracts>()
        // factions
        .init_resource::<GetFaction>()
        .init_resource::<ListFactions>()
        // fleet
        .init_resource::<CreateChart>()
        .init_resource::<CreateSurvey>()
        .init_resource::<DockShip>()
        .init_resource::<ExtractResources>()
        .init_resource::<GetMounts>()
        .init_resource::<GetShipCargo>()
        .init_resource::<GetShipCooldown>()
        .init_resource::<GetShipNav>()
        .init_resource::<GetShip>()
        .init_resource::<InstallMount>()
        .init_resource::<JettisonCargo>()
        .init_resource::<JumpShip>()
        .init_resource::<ListShips>()
        .init_resource::<NavigateShip>()
        .init_resource::<NegotiateContract>()
        .init_resource::<OrbitShip>()
        .init_resource::<PatchShipNav>()
        .init_resource::<PurchaseCargo>()
        .init_resource::<PurchaseShip>()
        .init_resource::<RefuelShip>()
        .init_resource::<RemoveMount>()
        .init_resource::<ScanShips>()
        .init_resource::<ScanSystems>()
        .init_resource::<ScanWaypoints>()
        .init_resource::<SellCargo>()
        .init_resource::<ShipRefine>()
        .init_resource::<TransferCargo>()
        .init_resource::<WarpShip>()
        // systems
        .init_resource::<GetJumpGate>()
        .init_resource::<GetMarket>()
        .init_resource::<GetShipyard>()
        .init_resource::<GetSystem>()
        .init_resource::<GetWaypoint>()
        .init_resource::<ListSystems>()
        .init_resource::<ListWaypointsInSystem>();
    }
}

pub const AD: &str = "https://api.spacetraders.io/v2/";

#[derive(Debug, Clone)]
pub struct QueryConf {
    pub limit: Option<core::num::NonZeroU8>,
    pub page: Option<core::num::NonZeroU8>,
}

pub trait TMinreqRequest {
    /// tries to create marker specific endpoint request
    fn try_create_minreq_request<B: serde::Serialize + std::fmt::Debug>(
        config: ClientConnectionConfig,
        body: &B,
        query: &Option<QueryConf>,
        args: Vec<String>,
    ) -> Result<minreq::Request, ClientError>;
}

fn dispatch_requests(
    new: ResMut<RequestsNew>,
    mut old: ResMut<RequestsOld>,
    connection_config: Res<ClientConnectionConfig>,
    mut buckets: ResMut<RateBucket>,
) {
    let pool = IoTaskPool::get();
    let mut working = core::mem::take(&mut *new.requests.lock().unwrap());

    working.append(&mut old.requests); // old are placed behind new
    working.sort_by(|a, b| a.rates.comp_rev(&b.rates)); // sorted: low...high; new, old

    let (mut normal, mut burst): (Vec<_>, Vec<_>) = working
        .into_iter()
        .partition(|request| request.rates.limit == RateLimit::Normal);

    while buckets.inner.peek(RateLimit::Normal) {
        // gets request from back, old high are consumed before new low
        let Some(request) = normal.pop() else {
            break;
        };

        let connection_config = connection_config.clone();

        pool.spawn(async move {
            let mut request = request;

            request
                .data
                .send_and_receive(connection_config, request.args);
        })
        .detach();

        buckets.inner.take(RateLimit::Normal);
    }

    while buckets.inner.peek(RateLimit::Burst) {
        // gets request from back, so old high are consumed before new low
        let Some(request) = burst.pop() else {
            break;
        };

        let connection_config = connection_config.clone();

        pool.spawn(async move {
            let mut request = request;

            request
                .data
                .send_and_receive(connection_config, request.args);
        })
        .detach();

        buckets.inner.take(RateLimit::Burst);
    }

    // extract leftover queue requests
    let mut retain = normal
        .drain(..)
        .filter(|request| request.rates.strategy == RateStrategy::Queued)
        .collect::<Vec<_>>();
    retain.append(
        &mut burst
            .drain(..)
            .filter(|request| request.rates.strategy == RateStrategy::Queued)
            .collect::<Vec<_>>(),
    );

    old.requests.append(&mut retain);
}

#[derive(Debug)]
/// represents all, ignoring panics, possible errors produced by this crate
pub enum ClientError {
    Request(RequestError),
    Respond(RespondError),
    Connection(minreq::Error),
}

#[derive(Debug, Resource, Clone)]
/// client config
pub struct ClientConnectionConfig {
    /// endpoind base path
    pub path: String,
    /// private token for auth
    pub bearer_token: Option<String>,
}

impl ClientConnectionConfig {
    /// creates new request builder from connection config
    pub fn new_builder<'a, B: serde::Serialize>(self) -> MinreqRequestBuilderUnready<'a, B> {
        MinreqRequestBuilderUnready::new(self.bearer_token, self.path)
    }
}

impl Default for ClientConnectionConfig {
    fn default() -> Self {
        Self {
            path: AD.to_string(),
            bearer_token: None,
        }
    }
}
