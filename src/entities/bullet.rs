use crate::{
    assets::AssetsState,
    audio::{sound_effect, SfxAssets},
    components::*,
    constants::SCREEN_HALF_WIDTH,
    events::BlastEvent,
    menus::Menu,
    screens::Screen,
    vfx::{explosion::create_explosion_vfx, VfxAssets},
    AppSystems, GameplaySystems, PausableSystems,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<BulletAssets>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct BulletAssets {
    #[asset(path = "images/vfx/bullet.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub bullet: Handle<Image>,
}
