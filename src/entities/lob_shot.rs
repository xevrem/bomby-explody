use crate::{
    assets::AssetsState, components::*, screens::Screen, AppSystems, GameplaySystems,
    PausableSystems,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<LobShotAssets>(),
    )
    .add_systems(
        Update,
        (arc_lob_shot)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

#[derive(AssetCollection, Resource)]
pub struct LobShotAssets {
    #[asset(image(sampler(filter = nearest)))]
    pub lob_shot: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 2))]
    pub lob_shot_layout: Handle<TextureAtlasLayout>,
}

pub fn create_lob_shot(assets: &LobShotAssets, spawn_pos: Vec2, target_pos: Vec2) -> impl Bundle {
    (
        Name::new("Lob Shot"),
        LobShot,
        StateScoped(Screen::Gameplay),
        Sprite {
            image: assets.lob_shot.clone(),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: assets.lob_shot_layout.clone(),
            }),
            custom_size: Some(Vec2::splat(96.0)),
            ..default()
        },
        Transform::from_translation(spawn_pos.extend(0.0)),
        AnimationConfig::new(0, 7, 24),
        Animating,
        TargetPosition {
            position: target_pos,
        },
    )
}

fn arc_lob_shot(mut commands: Commands) {
    //
}
