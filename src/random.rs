use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EntropyPlugin::<WyRand>::default());
}
