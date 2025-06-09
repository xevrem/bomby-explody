// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

// mod asset_tracking;
mod animation;
mod assets;
mod audio;
mod background;
mod components;
mod constants;
mod controlls;
mod entities;
mod events;
mod health;
mod input;
mod levels;
mod menus;
mod movement;
mod physics;
mod random;
mod screens;
mod spawners;
mod theme;
mod ui;
mod vfx;
// dev specific
#[cfg(feature = "dev")]
mod dev_tools;
// demo specifc
#[cfg(feature = "demo")]
mod demo;

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use screens::Screen;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bomby Explody".to_string(),
                        fit_canvas_to_parent: true,
                        resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                        // .with_scale_factor_override(2.0),
                        resizable: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );
        // 3rd party plugins
        app.add_plugins((
            assets::plugin,
            physics::plugin,
            input::plugin,
            random::plugin,
        ));

        // Add other plugins.
        app.add_plugins((
            animation::plugin,
            audio::plugin,
            background::plugin,
            components::plugin,
            controlls::plugin,
            entities::plugin,
            events::plugin,
            health::plugin,
            levels::plugin,
        ));

        app.add_plugins((
            menus::plugin,
            movement::plugin,
            screens::plugin,
            spawners::plugin,
            theme::plugin,
            ui::plugin,
            vfx::plugin,
            // dev specific
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            // demo specific
            #[cfg(feature = "demo")]
            demo::plugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::Events,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
        app.configure_sets(Update, GameplaySystems.run_if(in_state(Screen::Gameplay)));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// event response handling
    Events,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

/// A system set for systems that should only run while the scene is in Gameplay.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct GameplaySystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
