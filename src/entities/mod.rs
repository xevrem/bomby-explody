use bevy::prelude::*;

pub mod bombs;
pub mod character;
pub mod enemy;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bombs::plugin, character::plugin, enemy::plugin));
}
