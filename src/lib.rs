mod internal;

pub mod prelude {
    pub use super::internal::respond::response_received;
    pub use super::internal::respond::RespondError;

    pub use super::internal::endpoint::Endpoint;

    pub use super::internal::rate_limiter::RateLimit;
    pub use super::internal::rate_limiter::RatePriority;
    pub use super::internal::rate_limiter::RateStrategy;
    pub use super::internal::rate_limiter::Rates;

    pub use super::internal::client::ClientConnectionConfig;
    pub use super::internal::client::ClientError;
    pub use super::internal::client::ClientPlugin;
    pub use super::internal::client::QueryConf;

    pub use space_traders::models;
    pub use space_traders::models::*;

    pub use minreq::Method;

    pub use super::internal::minreq_request_builder::MinreqRequestBuilder;
    pub use super::internal::request::Authorization;

    #[cfg(feature = "offi-types")]
    pub use super::{
        endpoints,
        endpoints::*,
        internal::endpoints::{
            agents::get_public_agent::GetPublicAgent200Response, fleet::warp_ship::WarpShipRequest,
        },
    };
}

#[cfg(feature = "offi-types")]
pub mod endpoints {
    pub use super::internal::endpoints::get_status::GetStatus;

    pub use super::internal::endpoints::register_new_agent::RegisterNewAgent;

    pub use super::internal::endpoints::agents::get_agent::GetAgent;
    pub use super::internal::endpoints::agents::get_public_agent::GetPublicAgent;
    pub use super::internal::endpoints::agents::list_agents::ListAgents;

    pub use super::internal::endpoints::contracts::accept_contract::AcceptContract;
    pub use super::internal::endpoints::contracts::deliver_cargo_to_contract::DeliverCargoToContract;
    pub use super::internal::endpoints::contracts::fulfill_contract::FulfillContract;
    pub use super::internal::endpoints::contracts::get_contract::GetContract;
    pub use super::internal::endpoints::contracts::list_contracts::ListContracts;

    pub use super::internal::endpoints::factions::get_faction::GetFaction;
    pub use super::internal::endpoints::factions::list_factions::ListFactions;

    pub use super::internal::endpoints::systems::all_systems::AllSystems;
    pub use super::internal::endpoints::systems::get_jump_gate::GetJumpGate;
    pub use super::internal::endpoints::systems::get_market::GetMarket;
    pub use super::internal::endpoints::systems::get_shipyard::GetShipyard;
    pub use super::internal::endpoints::systems::get_system::GetSystem;
    pub use super::internal::endpoints::systems::get_waypoint::GetWaypoint;
    pub use super::internal::endpoints::systems::list_systems::ListSystems;
    pub use super::internal::endpoints::systems::list_waypoints_in_system::ListWaypointsInSystem;

    pub use super::internal::endpoints::fleet::create_chart::CreateChart;
    pub use super::internal::endpoints::fleet::create_survey::CreateSurvey;
    pub use super::internal::endpoints::fleet::dock_ship::DockShip;
    pub use super::internal::endpoints::fleet::extract_resources::ExtractResources;
    pub use super::internal::endpoints::fleet::extract_resources_with_survey::ExtractResourcesWithSurvey;
    pub use super::internal::endpoints::fleet::get_mounts::GetMounts;
    pub use super::internal::endpoints::fleet::get_ship::GetShip;
    pub use super::internal::endpoints::fleet::get_ship_cargo::GetShipCargo;
    pub use super::internal::endpoints::fleet::get_ship_cooldown::GetShipCooldown;
    pub use super::internal::endpoints::fleet::get_ship_nav::GetShipNav;
    pub use super::internal::endpoints::fleet::install_mount::InstallMount;
    pub use super::internal::endpoints::fleet::jettison_cargo::JettisonCargo;
    pub use super::internal::endpoints::fleet::jump_ship::JumpShip;
    pub use super::internal::endpoints::fleet::list_ships::ListShips;
    pub use super::internal::endpoints::fleet::navigate_ship::NavigateShip;
    pub use super::internal::endpoints::fleet::negotiate_contract::NegotiateContract;
    pub use super::internal::endpoints::fleet::orbit_ship::OrbitShip;
    pub use super::internal::endpoints::fleet::patch_ship_nav::PatchShipNav;
    pub use super::internal::endpoints::fleet::purchase_cargo::PurchaseCargo;
    pub use super::internal::endpoints::fleet::purchase_ship::PurchaseShip;
    pub use super::internal::endpoints::fleet::refuel_ship::RefuelShip;
    pub use super::internal::endpoints::fleet::remove_mount::RemoveMount;
    pub use super::internal::endpoints::fleet::scan_ships::ScanShips;
    pub use super::internal::endpoints::fleet::scan_systems::ScanSystems;
    pub use super::internal::endpoints::fleet::scan_waypoints::ScanWaypoints;
    pub use super::internal::endpoints::fleet::sell_cargo::SellCargo;
    pub use super::internal::endpoints::fleet::ship_refine::ShipRefine;
    pub use super::internal::endpoints::fleet::transfer_cargo::TransferCargo;
    pub use super::internal::endpoints::fleet::warp_ship::WarpShip;
}
