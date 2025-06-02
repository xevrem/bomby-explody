//! Spawn the main level.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    assets::AssetsState,
    // asset_tracking::LoadResource,
    audio::music,
    demo::player::{create_player, PlayerAssets},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    // app.register_type::<LevelAssets>();
    // app.load_resource::<LevelAssets>();
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::Loading).load_collection::<LevelAssets>(),
    );
}

// #[derive(Resource, Asset, Clone, Reflect)]
// #[reflect(Resource)]
#[derive(Resource, AssetCollection)]
pub struct LevelAssets {
    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    music: Handle<AudioSource>,
}

// impl FromWorld for LevelAssets {
//     fn from_world(world: &mut World) -> Self {
//         let assets = world.resource::<AssetServer>();
//         Self {
//             music: assets.load("audio/music/Fluffing A Duck.ogg"),
//         }
//     }
// }

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            create_player(400.0, &player_assets, &mut texture_atlas_layouts),
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )
        ],
    ));
}
