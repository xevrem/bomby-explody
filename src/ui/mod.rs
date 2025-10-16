use bevy::prelude::*;

mod player;
mod wave;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, wave::plugin));
}
