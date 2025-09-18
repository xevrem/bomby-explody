//! Spawn the main level.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
// use rand::prelude::*;

use crate::{
    audio::music,
    background::{create_background, BackgroundAssets},
    components::Level,
    entities::
        player::{create_player_character, CharacterAssets}
    ,
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
    character_assets: Res<CharacterAssets>,
    background_assets: Res<BackgroundAssets>,
    mut entropy: GlobalEntropy<WyRand>,
) {
    let background = create_background(&mut commands, &background_assets, &mut entropy);

    commands
        .spawn((
            Name::new("Level"),
            Level,
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Gameplay),
            children![
                // FIXME: this is dev hp
                create_player_character(&character_assets, 1000),
                (
                    Name::new("Gameplay Music"),
                    music(level_assets.music.clone())
                )
            ],
        ))
        .add_child(background);
}
