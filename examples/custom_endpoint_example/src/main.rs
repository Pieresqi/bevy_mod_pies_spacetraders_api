use bevy::{log::LogPlugin, prelude::*};
use bevy_mod_pies_spacetraders_api::prelude::*;

//lets create our own register endpoint

//define new type alias so its easier to write
// first type is for sending json data, second is for receiving json data
// specific type depends on server impl, you can use `()` to say it doesnt send and/or receive json data
// NOTE: you should create your own struct for generic par to evade endpoint collision
type MyCustomRegisterEndpoint = Endpoint<models::RegisterRequest, models::Register201Response>;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(ClientPlugin)
        // we need "register" our own endpond with bevy so it can be used in systems
        .init_resource::<MyCustomRegisterEndpoint>()
        .run();
}

// create new function which will make it easier to make request
// we can also define new trait and in it add fn like `set_request_custom` so its called: `endpoint.set_request_custom`
fn my_custom_endpoint_set_request(
    endpoint: &MyCustomRegisterEndpoint,
    request: models::RegisterRequest,
) {
    // what we specify there depends on server impl, 
    // MinreqRequestBuilder offers builder pattern so we can specifiy "optional" stuff like query, body, etc
    endpoint.send_request(
        Rates::default(),                           // rate limiter configuration
        MinreqRequestBuilder::new(
            Method::Post,                           // type of http request
            Authorization::Unnecessary              // needs auth
        )
                .set_additional_path("register")    // additional url path
                .set_body(request)                  // json data
                .set_query(QueryConfig::new())      // query param
    );
}

// example use in system
fn send_req_custom_endpoint(custom: Res<MyCustomRegisterEndpoint>) {
    my_custom_endpoint_set_request(
        &custom,
        models::RegisterRequest {
            faction: todo!(),
            symbol: todo!(),
            email: todo!(),
        },
    )
}
