use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::assets::AssetsState;

pub mod explosion;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<VfxAssets>(),
    );

    app.add_plugins(explosion::plugin);
}

#[derive(AssetCollection, Resource)]
pub struct VfxAssets {
    #[asset(path = "images/vfx/Fire_Explosion.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub explosion: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 1))]
    pub explosion_layout: Handle<TextureAtlasLayout>,
}
