use bevy::prelude::*;

pub(super) mod enemy;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((enemy::plugin));
}
