use bevy::prelude::*;

use crate::{components::Wave, screens::Screen, waves::WaveState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_initial_wave.run_if(in_state(WaveState::None)),
    );
}

fn spawn_initial_wave(mut commands: Commands, mut next_state: ResMut<NextState<WaveState>>) {
    commands.spawn((
        Name::new("wave"),
        Wave {
            level: 1,
            enemy_max: 5,
        },
    ));

    next_state.set(WaveState::Announce);
}
