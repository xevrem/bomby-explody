use bevy::prelude::*;

use crate::{
    components::Wave,
    events::SpawningDoneEvent,
    screens::Screen,
    spawners::enemies::create_enemy_spawner,
    waves::WaveState,
    AppSystems, GameplaySystems, PausableSystems,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_wave.run_if(in_state(WaveState::None)),
    ).add_systems(
        OnEnter(WaveState::Init),
        spawn_initial_wave
    );
}

fn spawn_wave(mut commands: Commands, mut next_state: ResMut<NextState<WaveState>>) {
    commands.spawn((
        Name::new("wave"),
        StateScoped(Screen::Gameplay),
        Wave {
            level: 1,
            limit: 5,
            max_at_once: 2,
        },
    ));

    next_state.set(WaveState::Announce);
}


fn spawn_initial_wave(mut commands: Commands,
    wave: Single<&Wave>,
    mut next_state: ResMut<NextState<WaveState>>
) {
    create_enemy_spawner(&mut commands, wave.limit, wave.max_at_once);

    next_state.set(WaveState::Running);
}
