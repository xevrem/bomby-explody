use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::prelude::*;

use crate::{
    assets::AssetsState,
    components::*,
    constants::{NUM_TILES_X, NUM_TILES_Y, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<BackgroundAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct BackgroundAssets {
    #[asset(path = "images/grasslands.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub background: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 32, rows = 23))]
    pub background_layout: Handle<TextureAtlasLayout>,
}

pub fn create_background(
    commands: &mut Commands,
    background_assets: &BackgroundAssets,

    entropy: &mut GlobalEntropy<WyRand>,
) -> Entity {
    commands
        .spawn((
            Name::new("Background"),
            Background,
            InheritedVisibility::VISIBLE,
            StateScoped(Screen::Gameplay),
            Transform::from_xyz(-SCREEN_WIDTH / 2. + 8., -SCREEN_HEIGHT / 2. + 8., 0.),
        ))
        .with_children(|builder| {
            for x in 0..NUM_TILES_X {
                for y in 0..NUM_TILES_Y {
                    let val = entropy.random_range(0..3);
                    builder.spawn(create_tile(
                        TILE_SIZE * x as f32,
                        TILE_SIZE * y as f32,
                        32 + val,
                        &background_assets,
                    ));
                }
            }
        })
        .id()
}

fn create_tile(x: f32, y: f32, index: usize, assets: &BackgroundAssets) -> impl Bundle {
    (
        Sprite {
            image: assets.background.clone(),
            texture_atlas: Some(TextureAtlas {
                index,
                layout: assets.background_layout.clone(),
            }),
            // darken grass
            color: Color::srgb(0.6, 0.6, 0.6),
            custom_size: Some(Vec2::splat(TILE_SIZE)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
    )
}
