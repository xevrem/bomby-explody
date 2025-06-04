use bevy::prelude::*;

pub mod enemy;
pub mod bombs;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bombs::plugin, enemy::plugin));
}
