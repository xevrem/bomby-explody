use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<AssetsState>();
    app.add_loading_state(
        LoadingState::new(AssetsState::LoadInitial).continue_to_state(AssetsState::InitialReady),
    );
    app.add_loading_state(
        LoadingState::new(AssetsState::LoadGameplay).continue_to_state(AssetsState::GameplayReady),
    );
    app.add_systems(OnEnter(AssetsState::InitialReady), || {
        info!("preload assets ready");
    });
    app.add_systems(OnEnter(AssetsState::GameplayReady), || {
        info!("game assets ready");
    });
    // app.add_systems(PreUpdate, load_early_assets);
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetsState {
    #[default]
    LoadInitial,
    InitialReady,
    LoadGameplay,
    GameplayReady,
}

// fn load_early_assets(mut next_state: ResMut<NextState<AssetsState>>) {
//     next_state.set(AssetsState::Preload);
// }
