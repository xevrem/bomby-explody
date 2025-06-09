use std::borrow::Cow;

use bevy::prelude::*;

use crate::{
    components::*, screens::Screen, theme::widget, AppSystems, GameplaySystems, PausableSystems,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), create_player_hp_ui);
    app.add_systems(
        Update,
        sync_player_hp_box
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

pub fn create_player_hp_ui(mut commands: Commands) {
    commands.spawn((
        widget::ul_ui_root("Player HP UI"),
        GlobalZIndex(2),
        StateScoped(Screen::Gameplay),
        children![(
            Name::new("hp ui grid"),
            Node {
                display: Display::Grid,
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),
                grid_template_columns: RepeatedGridTrack::px(2, 100.0),
                ..default()
            },
            children![
                (
                    widget::label("Health:"),
                    Node {
                        justify_self: JustifySelf::Start,
                        ..default()
                    }
                ),
                (
                    widget::label("100"),
                    Node {
                        justify_self: JustifySelf::Start,
                        ..default()
                    },
                    PlayerLabel,
                    HealthLabel,
                )
            ]
        )],
    ));
}

pub fn sync_player_hp_box(
    mut label: Single<&mut Text, (With<PlayerLabel>, With<HealthLabel>)>,
    player: Single<&Health, With<Player>>,
) {
    let val = player.current;
    label.0 = format!("{val}");
}
