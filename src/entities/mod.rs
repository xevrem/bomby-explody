use bevy::prelude::*;

pub mod bombs;
pub mod bullet;
pub mod enemy;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bombs::plugin, bullet::plugin, player::plugin, enemy::plugin));
}
