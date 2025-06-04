use crate::{assets::AssetsState, components::*, screens::Screen};
use avian2d::parry::simba::scalar::SupersetOf;
use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<PlayerControlls>();

    app.add_systems(OnEnter(Screen::Gameplay), create_controls);
    // create_controls
    //     .in_set(AppSystems::Update)
    //     .in_set(PausableSystems)
}

#[derive(InputContext)]
struct PlayerControlls;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub struct PlaceBomb;

fn create_controls(mut commands: Commands) {
    let mut actions = Actions::<PlayerControlls>::default();
    actions
        .bind::<PlaceBomb>()
        .to(MouseButton::Left)
        .with_conditions(Press::default());
    commands.spawn((
        Name::new("Player Controls"),
        actions,
        StateScoped(Screen::Gameplay),
    ));
}
