#![allow(clippy::type_complexity)]

pub mod random;

use bevy::prelude::*;
use random::RandomnessPlugin;


fn main() {
    let mut app = App::new();

    // External Plugins
    app.add_plugins((DefaultPlugins));

    #[cfg(all(feature = "debug", not(target_family = "wasm")))]
    app.add_plugins((
        bevy_remote::RemotePlugin::default(),
        bevy_remote::http::RemoteHttpPlugin::default(),
    ));

    // Internal Plugins
    app.add_plugins((RandomnessPlugin));

    app.run();
}
