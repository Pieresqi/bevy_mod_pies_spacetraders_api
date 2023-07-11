# Unofficial SpaceTraders API for Bevy

SpaceTraders API implemented for Bevy the game engine.

Consult official [Docs](https://spacetraders.stoplight.io/docs/spacetraders/) for APIs explanations.

Learm more about Bevy [here](https://bevyengine.org) and about SpaceTraders [here](https://spacetraders.io).

Issues / Pull requests / criticism / requests welcome.

## Features

- Simple and Ergonomic (I hope) API
- Integrated rate limiter
- Made for Bevy

## Simple Example

```rust
use bevy::{log::LogPlugin, prelude::*};
use bevy_mod_pies_spacetraders_api::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin::default())
        // we will need this, it sets up stuff
        .add_plugin(ClientPlugin)
        .add_system(add_token.in_schedule(CoreSchedule::Startup))
        .add_system(set_status.run_if(run_once()))
        .add_system(get_status.run_if(/* custom run condition is provided: */response_received::<GetStatus>()))
        .run();
}

fn add_token(mut config: ResMut<ClientConnectionConfig>) {
    // bearer token, almost every API needs it
    config.bearer_token = Some("Bearer XXXX".to_string());
}
// there's no need to specify 'markers::' but it makes it easy to know available apis
fn set_status(mut status: ResMut<markers::GetStatus>) {
    // setting rates is optional (defaults: RateLimit::Normal, RateStrategy::Queued, RatePriority::Normal)
    // will be reset after sending request
    status.add_rates(Rates {
        // we will use Burst limiter - up to 10 requests per second over 10 seconds
        limit: RateLimit::Burst,
        // request will be queued untill wa can send it
        strategy: RateStrategy::Queued,
        ..default()
    });
    // we can send request with this method, each API has it's own impl and will require different args
    status.set_request();
}

// each API is it's own Resource (on surface, interior mutability goes BRRRRR)
fn get_status(mut status: ResMut<markers::GetStatus>) {
    for status in status.write_unwrap().drain(..) {
        match status {
            Ok(status) => info!("{:?}", status),
            Err(error) => warn!("{:?}", error),
        }
    }
}
```

## Version Compatibility Table

|SpaceTraders|Bevy|Crate|
|-|-|-|
|`June 24, 2023`|`0.11.0`|`0.2.0`|
|`June 24, 2023`|`0.10.1`|`0.1.0`, `0.1.1`|

## License

Repo is dual licensed under `MIT` or `Apache-2.0` unless stated othervise.
