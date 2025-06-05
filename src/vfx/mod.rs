use bevy::prelude::*;

pub mod explosion;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(explosion::plugin);
}
