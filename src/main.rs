#![allow(clippy::type_complexity)]
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    // External Plugins
    app.add_plugins((DefaultPlugins));

    // Initialize Debugger
    #[cfg(all(feature = "debug", not(target_family = "wasm")))]
    app.add_plugins((
        bevy_remote::RemotePlugin::default(),
        bevy_remote::http::RemoteHttpPlugin::default(),
    ));

    // Initialize Global Random
    app.insert_resource(bevy_simple_random::ChaChaGlobalRng::default());

    // Internal Plugins
    app.add_plugins(());

    app.run();
}
