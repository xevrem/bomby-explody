//! Spawn the main level.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    audio::music,
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    entities::enemy::{create_enemy, EnemyAssets},
    screens::Screen,
};

#[derive(Resource, AssetCollection, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    music: Handle<AudioSource>,
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    enemy_assets: Res<EnemyAssets>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            create_enemy(
                &enemy_assets,
                0,
                // center right side of screen
                Vec2::new(SCREEN_WIDTH / 2.0, 0.0),
                Vec2::new(-1., 0.),
                0.1
            ),
            create_enemy(
                &enemy_assets,
                0,
                // center right side of screen
                Vec2::new(SCREEN_WIDTH / 2.0, 180.),
                Vec2::new(-1., 0.),
                0.15
            ),
            create_enemy(
                &enemy_assets,
                0,
                // center right side of screen
                Vec2::new(SCREEN_WIDTH / 2., -180.0),
                Vec2::new(-1., 0.),
                0.2
            ),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )
        ],
    ));
}
