use bevy::{log::LogPlugin, prelude::*};
use bevy_mod_pies_spacetraders_api::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        // we will need this, it sets up stuff
        .add_plugins(ClientPlugin)
        .add_systems(Startup, add_token)
        .add_systems(Update, set_status.run_if(run_once()))
        .add_systems(Update, get_status.run_if(/* custom run condition is provided: */response_received::<GetStatus>()))
        .run();
}

fn add_token(mut config: ResMut<ClientConnectionConfig>) {
    // bearer token, almost every API needs it
    config.set_bearer_token("XXX");
}
// there's no need to specify 'endpoints::' but it makes it easy to know available apis
fn set_status(mut status: Res<endpoints::GetStatus>) {
    // setting rates is optional (defaults: RateLimit::Normal, RateStrategy::Queued, RatePriority::Normal)
    // will be reset after sending request
    status.set_rates(Rates {
        // we will use Burst limiter - up to 10 requests per second over 10 seconds
        limit: RateLimit::Burst,
        // request will be queued untill wa can send it
        strategy: RateStrategy::Queued,
        ..default()
    });
    // we can send request with this method, each API has it's own impl and will require different args
    status.set_request();
}

// each API is it's own Resource
fn get_status(mut status: Res<endpoints::GetStatus>) {
    for status in status.get_receiver().try_iter() {
        match status {
            Ok(status) => info!("{:?}", status),
            Err(error) => warn!("{:?}", error),
        }
    }
}