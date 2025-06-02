//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
    assets::AssetsState,
    // asset_tracking::ResourceHandles,
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    // app.add_systems(OnEnter(AssetsState::AssetLoadingDone), asset_loading_done);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button("Play", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
}

// fn asset_loading_done(
//     mut next_state: ResMut<NextState<AssetsState>>,
//     mut next_screen: ResMut<NextState<Screen>>,
// ) {
//     next_state.set(AssetsState::Gameplay);
//     next_screen.set(Screen::Gameplay);
// }

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    assets_state: Res<State<AssetsState>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if assets_state.get() == &AssetsState::Ready {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
