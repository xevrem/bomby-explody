use crate::{
    assets::AssetsState,
    components::*,
    events::{BlastEvent, DamageEvent},
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
        apply_blast_damage
            .in_set(AppSystems::Events)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
    app.add_systems(
        Update,
        (handle_damaged, handle_dead)
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
        ScreenWrap,
        Transform::from_translation(position.extend(0.0)),
    )
}

fn apply_blast_damage(
    mut blast_reader: EventReader<BlastEvent>,
    mut damage_writer: EventWriter<DamageEvent>,
    enemy_query: Query<
        (Entity, &GlobalTransform),
        (
            With<Enemy>,
            With<Damageable>,
            With<Blastable>,
            Without<Dead>,
        ),
    >,
) -> Result {
    if !blast_reader.is_empty() {
        for blast_event in blast_reader.read() {
            // let blast_trans = blast_query.get(blast_event.source)?;
            for (enemy, enemy_trans) in &enemy_query {
                if enemy_trans
                    .translation()
                    .xy()
                    .distance(blast_event.location)
                    <= 100.0
                {
                    // blasted
                    damage_writer.write(DamageEvent {
                        target: enemy,
                        amount: 1,
                    });
                }
            }
        }
    }

    Ok(())
}

fn handle_damaged(
    mut commands: Commands,
    mut damaged_query: Query<
        (Entity, &mut Sprite, &mut Damaged, Option<&Moving>),
        (With<Enemy>, Without<Dead>),
    >,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut damaged, maybe_moving) in damaged_query {
        // stop movment if we damage it
        if maybe_moving.is_some() {
            commands.entity(entity).try_remove::<Moving>();
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
            commands
                .entity(entity)
                .remove::<Damaged>()
                // resume movement
                .insert(Moving);
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
