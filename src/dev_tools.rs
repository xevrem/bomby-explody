//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::screens::Screen;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub(super) enum DebugState {
    ON,
    #[default]
    OFF,
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<DebugState>()
        .add_systems(
            Update,
            (
                toggle_debug_on
                    .run_if(input_just_pressed(TOGGLE_KEY).and(in_state(DebugState::OFF))),
                toggle_debug_off
                    .run_if(input_just_pressed(TOGGLE_KEY).and(in_state(DebugState::ON))),
            ),
        )
        // setup bevy inspector
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new().run_if(in_state(DebugState::ON)))
        // Log `Screen` state transitions.
        .add_systems(Update, log_transitions::<Screen>)
        // Toggle the debug overlay for UI.
        .add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_debug_off(mut debug: ResMut<NextState<DebugState>>) {
    debug.set(DebugState::OFF);
}

fn toggle_debug_on(mut debug: ResMut<NextState<DebugState>>) {
    debug.set(DebugState::ON);
}
