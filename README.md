# Unofficial SpaceTraders API for Bevy

SpaceTraders API implemented for Bevy the game engine.

Consult official [Docs](https://spacetraders.stoplight.io/docs/spacetraders/) for APIs explanations.

Learm more about Bevy [here](https://bevyengine.org) and about SpaceTraders [here](https://spacetraders.io).

Issues / Pull requests / criticism / requests welcome.

## Features

- Simple and Ergonomic API
- Integrated rate limiter
- Made for Bevy
- Easily create new API (endpoints)

## Simple Example

```rust
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

fn set_status(status: Res<endpoints::GetStatus>) {
    // we can send request with this method, each API has it's own impl and will require different args
    status.set_request(Rates {
        // we will use Burst limiter - up to 10 requests per second over 10 seconds
        limit: RateLimit::Burst,
        // request will be queued untill wa can send it
        strategy: RateStrategy::Queued,
        ..default()
    });
}

// each API is it's own Resource
fn get_status(status: Res<endpoints::GetStatus>) {
    for status in status.get_receiver().try_iter() {
        match status {
            Ok(status) => info!("{:?}", status),
            Err(error) => warn!("{:?}", error),
        }
    }
}
```

## Version Compatibility Table

|Bevy|Crate|
|-|-|
|`0.11`|`0.5.0`, `0.4.0`, `0.3.0`, `0.2.0`|
|`0.10`|`0.1.1`, `0.1.0`|

## License

Repo is dual licensed under `MIT` or `Apache-2.0` unless stated othervise.
