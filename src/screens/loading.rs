//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{
    // asset_tracking::ResourceHandles,
    assets::AssetsState,
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Loading),
        (spawn_loading_screen, start_asset_loading),
    );

    app.add_systems(
        Update,
        enter_gameplay_screen
            .run_if(in_state(Screen::Loading).and(in_state(AssetsState::GameplayReady))),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(Screen::Loading),
        children![widget::label("Loading...")],
    ));
}
fn start_asset_loading(mut next_state: ResMut<NextState<AssetsState>>) {
    info!("start loading gameplay assets");
    next_state.set(AssetsState::LoadGameplay);
}

fn enter_gameplay_screen(mut next_screen: ResMut<NextState<Screen>>) {
    info!("loading screen transitioning to gameplay screen");
    next_screen.set(Screen::Gameplay);
}

// fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
//     resource_handles.is_all_done()
// }
