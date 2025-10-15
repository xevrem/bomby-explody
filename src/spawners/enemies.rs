use bevy::{asset::AssetIndex, prelude::*};
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::prelude::*;

use crate::{
    components::{
        AssetIdx, Bomber, Done, Enemy, Flying, Ground, Health, Level, Spawner, Speed, SubType,
        TargetDistance,
    },
    constants::{SCREEN_HALF_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH},
    entities::enemy::{create_enemy, EnemyAssets},
    screens::Screen,
    waves::WaveState,
    AppSystems, GameplaySystems, PausableSystems,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            tick_enemy_spawner::<Flying>,
            tick_enemy_spawner::<Ground>,
            tick_enemy_spawner::<Bomber>,
        )
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    )
    .add_systems(OnEnter(WaveState::Done), despawn_spawner);
}

pub fn create_enemy_spawner<T>(
    commands: &mut Commands,
    sub_type: T,
    asset_index: AssetIdx,
    limit: usize,
    max_at_once: usize,
    rate: f32,
    max_speed: f32,
    target_distance: TargetDistance,
) where
    T: Component + Clone,
{
    commands.spawn((
        Name::new("Enemy Spawner"),
        asset_index,
        Enemy,
        Spawner {
            all_spawned: false,
            limit,
            max_at_once,
            spawned: 0,
            timer: Timer::from_seconds(rate, TimerMode::Repeating),
        },
        Speed(max_speed),
        SubType::<T>(sub_type),
        StateScoped(Screen::Gameplay),
        target_distance,
    ));
}

fn tick_enemy_spawner<T>(
    mut commands: Commands,
    spawner_query: Query<
        (
            &mut Spawner,
            &SubType<T>,
            &AssetIdx,
            &Speed,
            &TargetDistance,
            Entity,
        ),
        (With<Enemy>, Without<Done>),
    >,
    level: Single<Entity, With<Level>>,
    enemy_query: Query<&Enemy, (With<T>, With<Health>)>,
    timer: Res<Time>,
    enemy_assets: Res<EnemyAssets>,
    mut entropy: GlobalEntropy<WyRand>,
) where
    T: Component + Clone,
{
    for (mut spawner, sub_type, asset_index, max_speed, target_distance, spawner_ent) in
        spawner_query
    {
        if enemy_query.iter().count() <= spawner.max_at_once
            && !spawner.all_spawned
            && spawner.spawned < spawner.limit
        {
            spawner.timer.tick(timer.delta());

            if spawner.timer.just_finished() {
                let half_height: f32 = SCREEN_HEIGHT / 2.0 - 64.0;
                // let speed: f32 = entropy.random_range(0.05..0.1);
                let speed: f32 = entropy.random_range(0.05..max_speed.0);
                let y_position: f32 = entropy.random_range(-half_height..half_height);
                let targ_dist: f32 = target_distance.0 + entropy.random_range(-50.0..50.0);
                let spawned = commands
                    .spawn(create_enemy(
                        sub_type.0.clone(),
                        &enemy_assets,
                        // starting asset index
                        asset_index.0,
                        // center right side of screen
                        Vec2::new(SCREEN_WIDTH / 2.0, y_position),
                        // move direction
                        Vec2::new(-1., 0.),
                        // move speed
                        speed,
                        // rough target engage distance
                        targ_dist,
                    ))
                    .id();
                commands.entity(level.entity()).add_child(spawned);

                // updoot spawner count
                spawner.spawned += 1;

                if spawner.spawned == spawner.limit {
                    spawner.all_spawned = true;
                    commands.entity(spawner_ent).insert(Done);
                }
            }
        }
    }
}

fn despawn_spawner(
    mut commands: Commands,
    spawn_query: Query<Entity, (With<Spawner>, With<Enemy>)>,
) {
    for spawner in spawn_query {
        commands.entity(spawner).despawn();
        info!("spawner despawned");
    }
}
