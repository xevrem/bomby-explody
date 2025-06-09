use std::borrow::Cow;

use bevy::prelude::*;

use crate::{
    components::*, screens::Screen, theme::widget, AppSystems, GameplaySystems, PausableSystems,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), create_hp_ui);
    app.add_systems(
        Update,
        sync_player_hp_box
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

pub fn create_hp_ui(mut commands: Commands) {
    commands.spawn((
        widget::ul_ui_root("Player HP UI"),
        GlobalZIndex(2),
        StateScoped(Screen::Gameplay),
        children![
            widget::label("Health:"),
            (widget::label("100"), PlayerLabel, HealthLabel,)
        ],
    ));
}

pub fn sync_player_hp_box(
    mut label: Single<&mut Text, (With<PlayerLabel>, With<HealthLabel>)>,
    player: Single<&Health, With<Player>>,
) {
    let val = player.current;
    label.0 = format!("{val}");
}
