use bevy::prelude::*;

use crate::{
    components::{AssetIdx, Dead, Enemy, Flying, Ground, Health, Spawner, Wave},
    events::EnemyDiedEvent,
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
        handle_enemy_died
            .run_if(in_state(WaveState::Running))
            .in_set(AppSystems::Events)
            .in_set(GameplaySystems)
            .in_set(PausableSystems),
    )
    .add_systems(OnEnter(WaveState::Done), setup_next_wave);
}

fn spawn_wave_config(mut commands: Commands, mut next_state: ResMut<NextState<WaveState>>) {
    commands.spawn((
        Name::new("wave"),
        StateScoped(Screen::Gameplay),
        Wave {
            level: 1,
            limit: 5,
            limit_growth: 2,
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
    create_enemy_spawner(
        &mut commands,
        Flying,
        AssetIdx(0),
        wave.limit,
        wave.max_at_once,
        1.0,
    );
    create_enemy_spawner(
        &mut commands,
        Ground,
        AssetIdx(14 * 4),
        wave.limit,
        wave.max_at_once,
        1.0,
    );

    next_state.set(WaveState::Running);
}

fn handle_enemy_died(
    spawn_query: Query<&Spawner, With<Enemy>>,
    enemy_query: Query<&Enemy, (With<Health>, Without<Dead>)>,
    mut event_reader: EventReader<EnemyDiedEvent>,
    mut next_state: ResMut<NextState<WaveState>>,
) {
    // an enemy died
    for _event in event_reader.read() {
        // if no enemies are left alive
        // and all enemy spawners have spawned all their entities
        if enemy_query.is_empty() && spawn_query.iter().all(|s| s.all_spawned) {
            // then say spawning is over
            info!("wave is done");
            next_state.set(WaveState::Done);
        }
    }
}

fn setup_next_wave(mut wave: Single<&mut Wave>, mut next_state: ResMut<NextState<WaveState>>) {
    // update wave config
    wave.level += 1;

    if wave.max_at_once < (wave.limit / 3) {
        wave.max_at_once += 1;
    } else {
        wave.limit += wave.limit_growth;
    }

    info!("next wave setup, announcing...");
    // set next wave state
    next_state.set(WaveState::Announce);
}
