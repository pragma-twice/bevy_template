use bevy::prelude::*;
pub use rand::SeedableRng;
pub use rand_chacha::ChaCha8Rng;

pub(super) fn randomness_plugin(&self, app: &mut App) {
    app.insert_resource(GlobalRng(ChaCha8Rng::seed_from_u64(
        web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .expect("time should go forward")
            .as_secs(),
    )));
}

#[derive(Resource, Deref, DerefMut)]
pub struct GlobalRng(ChaCha8Rng);
