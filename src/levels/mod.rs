use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    config::{ConfigureLoadingState, LoadingStateConfig},
    LoadingStateAppExt,
};
use level::LevelAssets;

use crate::assets::AssetsState;

pub(super) mod level;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<LevelAssets>(),
    );
}
