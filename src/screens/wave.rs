//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{theme::prelude::*, waves::WaveState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(WaveState::Announce), spawn_wave_screen);
}

fn spawn_wave_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Wave Screen"),
        StateScoped(WaveState::Announce),
        children![widget::header("Wave")],
    ));
}
