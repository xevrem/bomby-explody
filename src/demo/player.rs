//! Player-specific behavior.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_enhanced_input::{
    events::Fired,
    prelude::{
        Actions, BindingBuilder, DeadZone, Down, InputAction, InputContext, InputContextAppExt,
        Negate, Press, Pulse, Release, SmoothNudge,
    },
    preset::Cardinal,
};

use crate::{
    assets::AssetsState,
    demo::{
        animation::PlayerAnimation,
        movement::{MovementController, ScreenWrap},
    },
    screens::Screen,
    // AppSystems, PausableSystems,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::Loading).load_collection::<PlayerAssets>(),
    );

    app.add_input_context::<PlayerControlls>();
    // app.add_systems(OnEnter(Screen::Gameplay), bind_actions);

    // Record directional input as movement controls.
    // app.add_systems(
    //     Update,
    //     record_player_directional_input
    //         .in_set(AppSystems::RecordInput)
    //         .in_set(PausableSystems),
    // );
    // app.add_observer(player_idle);
    app.add_observer(player_movement);
}

#[derive(InputContext)]
struct PlayerControlls;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
struct Move;

#[derive(Debug, InputAction)]
#[input_action(output = bool, require_reset = true)]
struct Idle;

fn bind_actions() -> Actions<PlayerControlls> {
    info!("actions bound");
    let mut actions = Actions::<PlayerControlls>::default();
    actions
        .bind::<Move>()
        .to(Cardinal::wasd_keys());
        // .with_modifiers((
        //     // Modifiers applied at the action level.
        //     DeadZone::default(), // Normalizes movement.
        //                          SmoothNudge::default(), // Smoothes movement.
        // ));
    // actions
    //     .bind::<Idle>()
    //     // .to((
    //     //     KeyCode::KeyW.with_conditions(Release::default()),
    //     //     KeyCode::KeyA.with_conditions(Release::default()),
    //     //     KeyCode::KeyS.with_conditions(Release::default()),
    //     //     KeyCode::KeyD.with_conditions(Release::default()),
    //     // ))
    //     .to(Cardinal::wasd_keys())
    //     .with_conditions(Release::default());

    actions
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

/// The player character.
pub fn create_player(
    max_speed: f32,
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player,
        bind_actions(),
        Sprite {
            image: player_assets.ducky.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.get_atlas_index(),
            }),
            ..default()
        },
        Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
        player_animation,
    )
}

// fn record_player_directional_input(
//     input: Res<ButtonInput<KeyCode>>,
//     mut controller_query: Query<&mut MovementController, With<Player>>,
// ) {
//     // Collect directional input.
//     let mut intent = Vec2::ZERO;
//     if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
//         intent.y += 1.0;
//     }
//     if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
//         intent.y -= 1.0;
//     }
//     if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
//         intent.x -= 1.0;
//     }
//     if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
//         intent.x += 1.0;
//     }

//     // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
//     // This should be omitted if the input comes from an analog stick instead.
//     let intent = intent.normalize_or_zero();

//     // Apply movement intent to controllers.
//     for mut controller in &mut controller_query {
//         controller.intent = intent;
//     }
// }

fn player_movement(
    trigger: Trigger<Fired<Move>>,
    controller_query: Query<&mut MovementController, With<Player>>,
) {
    // info!("player movement");
    for mut controller in controller_query {
        controller.intent = trigger.value;
    }
}

fn player_idle(
    trigger: Trigger<Fired<Idle>>,
    controller_query: Query<&mut MovementController, With<Player>>,
) {
    info!("player idle");
    for mut controller in controller_query {
        controller.intent = Vec2::ZERO;
    }
}

#[derive(Resource, AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "images/ducky.png")]
    #[asset(image(sampler(filter = nearest)))]
    ducky: Handle<Image>,
    #[asset(
        paths(
            "audio/sound_effects/step1.ogg",
            "audio/sound_effects/step2.ogg",
            "audio/sound_effects/step3.ogg",
            "audio/sound_effects/step4.ogg"
        ),
        collection(typed)
    )]
    pub steps: Vec<Handle<AudioSource>>,
}

// impl FromWorld for PlayerAssets {
//     fn from_world(world: &mut World) -> Self {
//         let assets = world.resource::<AssetServer>();
//         Self {
//             ducky: assets.load_with_settings(
//                 "images/ducky.png",
//                 |settings: &mut ImageLoaderSettings| {
//                     // Use `nearest` image sampling to preserve pixel art style.
//                     settings.sampler = ImageSampler::nearest();
//                 },
//             ),
//             steps: vec![
//                 assets.load("audio/sound_effects/step1.ogg"),
//                 assets.load("audio/sound_effects/step2.ogg"),
//                 assets.load("audio/sound_effects/step3.ogg"),
//                 assets.load("audio/sound_effects/step4.ogg"),
//             ],
//         }
//     }
// }
