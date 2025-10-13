use bevy::prelude::*;

pub mod bombs;
pub mod bullet;
pub mod enemy;
pub mod lob_shot;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bombs::plugin, bullet::plugin, lob_shot::plugin, player::plugin, enemy::plugin));
}
