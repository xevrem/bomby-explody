use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::prelude::*;

use crate::{
    components::*,
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    entities::enemy::{create_enemy, EnemyAssets},
    screens::Screen,
    AppSystems, GameplaySystems, PausableSystems,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        tick_enemy_spawner
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
    app.add_systems(OnEnter(Screen::Gameplay), create_enemy_spawner);
}

pub fn create_enemy_spawner(mut commands: Commands) {
    commands.spawn((
        Name::new("Enemy Spawner"),
        Enemy,
        Spawner {
            max: 5,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
        StateScoped(Screen::Gameplay),
    ));
}

fn tick_enemy_spawner(
    mut commands: Commands,
    mut spawner: Single<&mut Spawner, With<Enemy>>,
    level: Single<Entity, With<Level>>,
    enemy_query: Query<&Enemy>,
    timer: Res<Time>,
    enemy_assets: Res<EnemyAssets>,
    mut entropy: GlobalEntropy<WyRand>,
) {
    if enemy_query.iter().count() <= spawner.max {
        spawner.timer.tick(timer.delta());

        if spawner.timer.just_finished() {
            let half_height: f32 = SCREEN_HEIGHT / 2.0 - 64.0;
            let speed: f32 = entropy.random_range(0.1..0.2);
            let y_position: f32 = entropy.random_range(-half_height..half_height);
            let spawned = commands
                .spawn(create_enemy(
                    &enemy_assets,
                    // starting asset index
                    0,
                    // center right side of screen
                    Vec2::new(SCREEN_WIDTH / 2.0, y_position),
                    // move direction
                    Vec2::new(-1., 0.),
                    // move speed
                    speed,
                ))
                .id();
            commands.entity(level.entity()).add_child(spawned);
        }
    }
}
