use crate::{
    assets::AssetsState, components::*, constants::SCREEN_HALF_HEIGHT, events::DamageEvent,
    AppSystems, GameplaySystems, PausableSystems,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<EnemyAssets>(),
    );
    app.add_systems(
        Update,
        (
            handle_damaged,
            handle_dead,
            switch_to_attack_player,
            move_to_player,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

#[derive(AssetCollection, Resource)]
pub struct EnemyAssets {
    #[asset(path = "images/enemies.png")]
    #[asset(image(sampler(filter = nearest)))]
    enemies: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 30, tile_size_y = 30, columns = 4, rows = 48))]
    layout: Handle<TextureAtlasLayout>,
}
pub fn create_enemy(
    enemy_assets: &EnemyAssets,
    index: usize,
    position: Vec2,
    movement: Vec2,
    speed_percent: f32,
) -> impl Bundle {
    (
        Name::new("Enemy"),
        AnimationConfig::new(index, 4, 4),
        Animating,
        Blastable,
        Damageable,
        Enemy,
        Health { current: 10 },
        MovementConfig::from_vec2(movement).with_speed_as_screen_width_percent(speed_percent),
        Moving,
        Sprite {
            image: enemy_assets.enemies.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: enemy_assets.layout.clone(),
                index,
                ..default()
            }),
            custom_size: Some(Vec2::splat(30.0 * 3.0)),
            ..default()
        },
        // ScreenWrap,
        Transform::from_translation(position.extend(0.0)),
    )
}

fn handle_damaged(
    mut commands: Commands,
    mut damaged_query: Query<
        (
            Entity,
            &mut Sprite,
            &mut Damaged,
            Option<&Moving>,
            Option<&Attacking>,
            Option<&WasMoving>,
            Option<&WasAttacking>,
        ),
        (With<Enemy>, Without<Dead>),
    >,
    time: Res<Time>,
) {
    for (
        entity,
        mut sprite,
        mut damaged,
        maybe_moving,
        maybe_attacking,
        maybe_was_moving,
        maybe_was_attacking,
    ) in &mut damaged_query
    {
        // stop movment if we damage it
        if maybe_moving.is_some() {
            commands
                .entity(entity)
                .try_remove::<Moving>()
                .insert_if_new(WasMoving);
        }

        if maybe_attacking.is_some() {
            commands
                .entity(entity)
                .try_remove::<Attacking>()
                .insert_if_new(WasAttacking);
        }

        damaged.timer.tick(time.delta());
        let remaining = (damaged.timer.remaining_secs() * 10.0) as u32;
        if remaining % 2 == 0 {
            sprite.color = Color::srgb(1.0, 0.0, 0.0);
        } else {
            sprite.color = Color::srgb(1.0, 1.0, 1.0);
        }

        if damaged.timer.just_finished() {
            sprite.color = Color::srgb(1.0, 1.0, 1.0);
            if maybe_was_moving.is_some() {
                commands
                    .entity(entity)
                    .remove::<Damaged>()
                    .remove::<WasMoving>()
                    // resume movement
                    .insert(Moving);
            }
            if maybe_was_attacking.is_some() {
                commands
                    .entity(entity)
                    .remove::<Damaged>()
                    .remove::<WasAttacking>()
                    // resume attacking
                    .insert(Attacking);
            }
        }
    }
}

fn handle_dead(
    mut commands: Commands,
    mut dead_query: Query<(Entity, &mut Sprite, &mut Dead, Option<&Moving>), With<Enemy>>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut dead, maybe_moving) in &mut dead_query {
        // stop movment if dead it
        if maybe_moving.is_some() {
            commands.entity(entity).try_remove::<Moving>();
        }

        dead.timer.tick(time.delta());

        let frac = dead.timer.fraction_remaining();
        sprite.color = Color::srgba(1.0, 1.0, 1.0, frac);

        if dead.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn switch_to_attack_player(
    mut commands: Commands,
    enemy_query: Query<
        (Entity, &GlobalTransform),
        (
            With<Enemy>,
            Without<Player>,
            Without<Dead>,
            Without<Attacking>,
        ),
    >,
    player: Single<&GlobalTransform, (With<Player>, Without<Enemy>)>,
) {
    let player_position = player.translation().xy();
    for (enemy, enemy_trans) in enemy_query {
        let enemy_position = enemy_trans.translation().xy();
        let distance = enemy_position.distance(player_position);
        if distance <= SCREEN_HALF_HEIGHT {
            let time_to_attack = (distance / 200.0) / 2.0;

            commands
                .entity(enemy)
                .insert((
                    Attacking,
                    Countdown {
                        timer: Timer::from_seconds(time_to_attack, TimerMode::Once),
                    },
                    TargetPosition {
                        position: player_position,
                    },
                    EaseFunc(EasingCurve::new(
                        enemy_position,
                        player_position,
                        EaseFunction::BackIn,
                    )),
                ))
                .remove::<Moving>();
        }
    }
}

fn move_to_player(
    mut commands: Commands,
    mut enemy_query: Query<
        (Entity, &mut Transform, &mut Countdown, &EaseFunc<Vec2>),
        (With<Enemy>, With<Attacking>, Without<Dead>),
    >,
    player: Single<Entity, With<Player>>,
    time: Res<Time>,
    mut damage_writer: EventWriter<DamageEvent>,
) {
    for (enemy, mut trans, mut countdown, ease) in &mut enemy_query {
        countdown.timer.tick(time.delta());
        if countdown.timer.just_finished() {
            // kill enemy
            commands
                .entity(enemy)
                .remove::<TargetPosition>()
                .remove::<Attacking>()
                .insert_if_new(Dead {
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                });

            // spawn damage event
            damage_writer.write(DamageEvent {
                target: player.entity(),
                amount: 1,
            });
        } else {
            if let Some(new_pos) = ease.0.sample(countdown.timer.fraction()) {
                trans.translation = new_pos.extend(0.0);
            }
        }
    }
}
