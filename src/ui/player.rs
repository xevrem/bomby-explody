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
        Name::new("Player HP UI"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
        GlobalZIndex(2),
        StateScoped(Screen::Gameplay),
        children![(
            Name::new("hp ui grid"),
            Node {
                display: Display::Flex,
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
                        justify_self: JustifySelf::End,
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
