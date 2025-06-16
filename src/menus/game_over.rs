//! The pause menu.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::GameOver), spawn_game_over_menu);
}

fn spawn_game_over_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("GameOver Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::GameOver),
        children![
            widget::header("Game Over :("),
            widget::button("Quit to title", quit_to_title),
        ],
    ));
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
