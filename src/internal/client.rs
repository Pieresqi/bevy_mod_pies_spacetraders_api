use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_ecs::{
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_tasks::IoTaskPool;

use super::{
    endpoints::{
        agents::{get_agent::GetAgent, get_public_agent::GetPublicAgent, list_agents::ListAgents},
        contracts::{
            accept_contract::AcceptContract, deliver_cargo_to_contract::DeliverCargoToContract,
            fulfill_contract::FulfillContract, get_contract::GetContract,
            list_contracts::ListContracts,
        },
        factions::{get_faction::GetFaction, list_factions::ListFactions},
        fleet::{
            create_chart::CreateChart, create_survey::CreateSurvey, dock_ship::DockShip,
            extract_resources::ExtractResources,
            extract_resources_with_survey::ExtractResourcesWithSurvey, get_mounts::GetMounts,
            get_ship::GetShip, get_ship_cargo::GetShipCargo, get_ship_cooldown::GetShipCooldown,
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
            all_systems::AllSystems, get_jump_gate::GetJumpGate, get_market::GetMarket,
            get_shipyard::GetShipyard, get_system::GetSystem, get_waypoint::GetWaypoint,
            list_systems::ListSystems, list_waypoints_in_system::ListWaypointsInSystem,
        },
    },
    rate_limiter::{replenish_buckets, RateBucket, RateLimit, RateStrategy},
    request::{ChannelRequestHolder, RequestsToBeProcessed},
    respond::RespondError,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            dispatch_requests.run_if(
                move |new: Res<ChannelRequestHolder>,
                      old: Res<RequestsToBeProcessed>,
                      bucket: Res<RateBucket>| {
                    (!new.receiver.is_empty() || !old.requests.is_empty())
                        && (bucket.inner.peek(RateLimit::Normal)
                            || bucket.inner.peek(RateLimit::Burst))
                },
            ),
        )
        .add_systems(Update, replenish_buckets)
        .init_resource::<RequestsToBeProcessed>()
        .init_resource::<ChannelRequestHolder>()
        .init_resource::<RateBucket>()
        .init_resource::<ClientConnectionConfig>();

        #[cfg(feature = "offi-types")]
        app.init_resource::<GetStatus>()
            .init_resource::<RegisterNewAgent>()
            // agents
            .init_resource::<GetAgent>()
            .init_resource::<ListAgents>()
            .init_resource::<GetPublicAgent>()
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
            .init_resource::<ExtractResourcesWithSurvey>()
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
            .init_resource::<ListWaypointsInSystem>()
            .init_resource::<AllSystems>();
    }
}

pub const BASE_PATH: &str = "https://api.spacetraders.io/v2/";

#[derive(Debug)]
pub struct QueryConf {
    pub limit: Option<core::num::NonZeroU8>,
    pub page: Option<core::num::NonZeroU8>,
}

fn dispatch_requests(
    new: ResMut<ChannelRequestHolder>,
    mut old: ResMut<RequestsToBeProcessed>,
    connection_config: Res<ClientConnectionConfig>,
    mut buckets: ResMut<RateBucket>,
) {
    let pool = IoTaskPool::get();
    let mut working = new.receiver.try_iter().collect::<Vec<_>>();

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
            request.data.send_and_receive(connection_config);
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
            request.data.send_and_receive(connection_config);
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
    Respond(RespondError),
    Connection(minreq::Error),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = (match self {
                Self::Respond(s) => s.to_string(),
                Self::Connection(s) => s.to_string(),
            }).to_string();

        write!(f, "{display}")
    }
}

impl std::error::Error for ClientError {}

#[derive(Debug, Resource, Clone)]
/// client config
pub struct ClientConnectionConfig {
    /// endpoind base path
    pub path: String,
    /// private token for auth
    pub bearer_token: String,
}

impl ClientConnectionConfig {
    pub fn set_endpoint_path<I: Into<String>>(&mut self, end_path: I) {
        self.path = end_path.into();
        if !self.path.ends_with('/') {
            self.path.push('/');
        }
    }

    pub fn set_bearer_token<I: Into<String>>(&mut self, bear_token: I) {
        self.bearer_token = bear_token.into();
        if !self.bearer_token.starts_with("Bearer ") {
            self.bearer_token = format!("Bearer {}", self.bearer_token);
        }
    }

    pub fn get_endpoint_path(&self) -> &str {
        &self.path
    }

    pub fn get_bearer_token(&self) -> &str {
        &self.bearer_token
    }
}

impl Default for ClientConnectionConfig {
    fn default() -> Self {
        Self {
            path: BASE_PATH.to_string(),
            bearer_token: String::new(),
        }
    }
}
