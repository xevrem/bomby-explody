use bevy::prelude::*;
mod enemies;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((self::enemies::plugin));
}
