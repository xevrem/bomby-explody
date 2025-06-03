//! Spawn the main level.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    audio::music,
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
            create_enemy(&enemy_assets, 0, Vec2::new(0., 0.)),
            //create_player(400.0, &player_assets, &mut texture_atlas_layouts),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )
        ],
    ));
}
