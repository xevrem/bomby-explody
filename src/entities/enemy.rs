use crate::{
    assets::AssetsState,
    components::*,
    events::{BlastEvent, DamageEvent},
    AppSystems, GameplaySystems, PausableSystems,
};
use avian2d::parry::simba::scalar::SupersetOf;
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
        Enemy,
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
        AnimationConfig::new(index, 4, 4),
        Animating,
        MovementConfig::from_vec2(movement).with_speed_as_screen_width_percent(speed_percent),
        Moving,
        ScreenWrap,
        Transform::from_translation(position.extend(0.0)),
    )
}

fn apply_blast_damage(
    mut commands: Commands,
    mut blast_reader: EventReader<BlastEvent>,
    mut damage_writer: EventWriter<DamageEvent>,
    blast_query: Query<&GlobalTransform, Without<Enemy>>,
    enemy_query: Query<
        (Entity, &GlobalTransform),
        (With<Enemy>, With<Damageable>, With<Blastable>),
    >,
) -> Result {
    if !blast_reader.is_empty() {
        for blast_event in blast_reader.read() {
            let blast_trans = blast_query.get(blast_event.source)?;
            for (enemy, enemy_trans) in &enemy_query {
                if enemy_trans
                    .translation()
                    .distance(blast_trans.translation())
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
