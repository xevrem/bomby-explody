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
        spawn_wave_config.run_if(in_state(WaveState::None)),
    )
    .add_systems(OnEnter(WaveState::Init), spawn_wave)
    .add_systems(
        Update,
        handle_spawning_done
            .in_set(AppSystems::Events)
            .in_set(GameplaySystems)
            .in_set(PausableSystems),
    )
}

fn spawn_wave_config(mut commands: Commands, mut next_state: ResMut<NextState<WaveState>>) {
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

fn spawn_wave(
    mut commands: Commands,
    wave: Single<&Wave>,
    mut next_state: ResMut<NextState<WaveState>>,
) {
    create_enemy_spawner(&mut commands, wave.limit, wave.max_at_once);

    next_state.set(WaveState::Running);
}

fn handle_spawning_done(
    event_reader: EventReader<SpawningDoneEvent>,
    mut next_state: ResMut<NextState<WaveState>>,
) {
    if !event_reader.is_empty() {
        // wave is done
        next_state.set(WaveState::Done);
    }
}
