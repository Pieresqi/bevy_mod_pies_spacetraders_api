mod internal;

pub mod prelude {
    pub use super::internal::respond::response_received;
    pub use super::internal::respond::RespondError;

    pub use super::internal::request::RequestError;

    pub use super::internal::marker::Marker;

    pub use super::internal::rate_limiter::RateLimit;
    pub use super::internal::rate_limiter::RatePriority;
    pub use super::internal::rate_limiter::RateStrategy;
    pub use super::internal::rate_limiter::Rates;

    pub use super::internal::client::ClientConnectionConfig;
    pub use super::internal::client::ClientPlugin;

    pub use super::internal::markers::fleet::warp_ship::WarpShip;

    pub use openapi::models;
    pub use openapi::models::*;

    pub use super::markers;
    pub use super::markers::*;
}

pub mod markers {
    pub use super::internal::markers::get_status::GetStatus;

    pub use super::internal::markers::register_new_agent::RegisterNewAgent;

    pub use super::internal::markers::agents::get_agent::GetAgent;

    pub use super::internal::markers::contracts::accept_contract::AcceptContract;
    pub use super::internal::markers::contracts::deliver_cargo_to_contract::DeliverCargoToContract;
    pub use super::internal::markers::contracts::fulfill_contract::FulfillContract;
    pub use super::internal::markers::contracts::get_contract::GetContract;
    pub use super::internal::markers::contracts::list_contracts::ListContracts;

    pub use super::internal::markers::factions::get_faction::GetFaction;
    pub use super::internal::markers::factions::list_factions::ListFactions;

    pub use super::internal::markers::systems::get_jump_gate::GetJumpGate;
    pub use super::internal::markers::systems::get_market::GetMarket;
    pub use super::internal::markers::systems::get_shipyard::GetShipyard;
    pub use super::internal::markers::systems::get_system::GetSystem;
    pub use super::internal::markers::systems::get_waypoint::GetWaypoint;
    pub use super::internal::markers::systems::list_systems::ListSystems;
    pub use super::internal::markers::systems::list_waypoints_in_system::ListWaypointsInSystem;

    pub use super::internal::markers::fleet::create_chart::CreateChart;
    pub use super::internal::markers::fleet::create_survey::CreateSurvey;
    pub use super::internal::markers::fleet::dock_ship::DockShip;
    pub use super::internal::markers::fleet::extract_resources::ExtractResources;
    pub use super::internal::markers::fleet::get_mounts::GetMounts;
    pub use super::internal::markers::fleet::get_ship::GetShip;
    pub use super::internal::markers::fleet::get_ship_cargo::GetShipCargo;
    pub use super::internal::markers::fleet::get_ship_cooldown::GetShipCooldown;
    pub use super::internal::markers::fleet::get_ship_nav::GetShipNav;
    pub use super::internal::markers::fleet::install_mount::InstallMount;
    pub use super::internal::markers::fleet::jettison_cargo::JettisonCargo;
    pub use super::internal::markers::fleet::jump_ship::JumpShip;
    pub use super::internal::markers::fleet::list_ships::ListShips;
    pub use super::internal::markers::fleet::navigate_ship::NavigateShip;
    pub use super::internal::markers::fleet::negotiate_contract::NegotiateContract;
    pub use super::internal::markers::fleet::orbit_ship::OrbitShip;
    pub use super::internal::markers::fleet::patch_ship_nav::PatchShipNav;
    pub use super::internal::markers::fleet::purchase_cargo::PurchaseCargo;
    pub use super::internal::markers::fleet::purchase_ship::PurchaseShip;
    pub use super::internal::markers::fleet::refuel_ship::RefuelShip;
    pub use super::internal::markers::fleet::remove_mount::RemoveMount;
    pub use super::internal::markers::fleet::scan_ships::ScanShips;
    pub use super::internal::markers::fleet::scan_systems::ScanSystems;
    pub use super::internal::markers::fleet::scan_waypoints::ScanWaypoints;
    pub use super::internal::markers::fleet::sell_cargo::SellCargo;
    pub use super::internal::markers::fleet::ship_refine::ShipRefine;
    pub use super::internal::markers::fleet::transfer_cargo::TransferCargo;
    pub use super::internal::markers::fleet::warp_ship::WarpShip;
}

pub mod _internal {
    // be free
    pub use super::internal::*;
}
