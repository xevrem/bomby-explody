use crate::{assets::AssetsState, components::*};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<EnemyAssets>(),
    );
}

pub fn create_enemy(enemy_assets: &EnemyAssets, index: usize, position: Vec2) -> impl Bundle {
    (
        Name::new("Enemy"),
        Enemy,
        Sprite::from_atlas_image(
            enemy_assets.enemies.clone(),
            TextureAtlas {
                layout: enemy_assets.layout.clone(),
                index,
                ..default()
            },
        ),
        AnimationConfig::new(index, 4, 4),
        Animating,
        MovementConfig::from_vec2(Vec2::new(0.0, 1.0)).with_speed_as_screen_height_percent(0.1),
        Moving,
        Transform {
            translation: position.extend(0.0),
            scale: Vec3::splat(3.0),
            ..default()
        },
    )
}

#[derive(AssetCollection, Resource)]
pub struct EnemyAssets {
    #[asset(path = "images/enemies.png")]
    #[asset(image(sampler(filter = nearest)))]
    enemies: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 30, tile_size_y = 30, columns = 4, rows = 48))]
    layout: Handle<TextureAtlasLayout>,
}
