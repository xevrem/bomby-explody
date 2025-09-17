//! The game's wave states

mod wave;

use bevy::prelude::*;

/// The game's main wave states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum WaveState {
    #[default]
    None,
    Announce,
    Init,
    New,
    Running,
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<WaveState>().add_plugins((wave::plugin,));
}
