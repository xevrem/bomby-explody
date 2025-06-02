use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<AssetsState>();
    app.add_loading_state(
        LoadingState::new(AssetsState::Unloaded).continue_to_state(AssetsState::Preloaded),
    );
    app.add_loading_state(
        LoadingState::new(AssetsState::Loading).continue_to_state(AssetsState::Ready),
    );
    app.add_systems(OnEnter(AssetsState::Preloaded), || {
        info!("preload assets state ready");
    });
    app.add_systems(OnEnter(AssetsState::Ready), || {
        info!("assets state ready");
    });
    // app.add_systems(PreUpdate, load_early_assets);
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetsState {
    #[default]
    Unloaded,
    Preloaded,
    Loading,
    Ready,
}

// fn load_early_assets(mut next_state: ResMut<NextState<AssetsState>>) {
//     next_state.set(AssetsState::Preload);
// }
