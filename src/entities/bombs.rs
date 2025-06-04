use crate::{
    assets::AssetsState, components::*, controlls::PlaceBomb, screens::Screen, AppSystems,
    PausableSystems,
};
use avian2d::parry::simba::scalar::SupersetOf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<BombAssets>(),
    );
    app.add_systems(OnEnter(Screen::Gameplay), add_click_to_spawn_observer);
    app.add_systems(
        Update,
        (bomb_timer_countdown, despawn_explosion).run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),
    );
}

fn bomb_timer_countdown(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Bomb, &GlobalTransform)>,
    time: Res<Time>,
    assets: Res<BombAssets>
) {
    for (entity, mut bomb, transform) in &mut query {
        bomb.timer.tick(time.delta());
        if bomb.timer.just_finished() {
            // BOOM

            // destroy
            commands.entity(entity).despawn_recursive();

            // make splosion
            commands.spawn(create_explosion(&assets, transform.translation().truncate()));
        }
    }
}

pub fn create_bomb(assets: &BombAssets, position: Vec2) -> impl Bundle {
    info!("create bomb at {}", position);
    (
        Name::new("Bomb"),
        Bomb {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        },
        StateScoped(Screen::Gameplay),
        Sprite {
            image: assets.ball.clone(),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: assets.ball_layout.clone(),
                ..default()
            }),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::from_translation(position.extend(0.0).clone()),
        AnimationConfig::new(0, 8, 6),
        Animating,
    )
}

#[derive(AssetCollection, Resource)]
pub struct BombAssets {
    #[asset(path = "images/vfx/Charge_Fire.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub charge: Handle<Image>,
    #[asset(path = "images/vfx/Fire_Explosion.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub explosion: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 1))]
    pub explosion_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "images/vfx/Lavaball.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub ball: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 2))]
    pub ball_layout: Handle<TextureAtlasLayout>,
}

fn add_click_to_spawn_observer(mut commands: Commands) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        Observer::new(place_bomb_on_click),
    ));
}

fn place_bomb_on_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    screen_state: Res<State<Screen>>,
    assets_state: Res<State<AssetsState>>,
    assets: Res<BombAssets>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    if screen_state.get() == &Screen::Gameplay && assets_state.get() == &AssetsState::GameplayReady
    {
        // info!("clicky {}", trigger.pointer_location.position);
        let (camera, camera_trans) = *camera_query;
        if let Ok(location) =
            camera.viewport_to_world_2d(camera_trans, trigger.pointer_location.position)
        {
            // let location = trigger.pointer_location.position;
            commands.spawn(create_bomb(&assets, location));
        }
    }
}

fn create_explosion(assets: &BombAssets, location: Vec2) -> impl Bundle {
    (
        Name::new("Explosion"),
        Explosion {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
        StateScoped(Screen::Gameplay),
        Sprite {
            image: assets.explosion.clone(),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: assets.explosion_layout.clone(),
                ..default()
            }),
            custom_size: Some(Vec2::splat(96.0)),
            ..default()
        },
        Transform::from_translation(location.extend(0.0)),
        AnimationConfig::new(0, 6, 12),
        Animating,
    )
}

fn despawn_explosion(mut commands: Commands, mut query: Query<(Entity, &mut Explosion)>, time: Res<Time> ) {
    for (entity, mut explosion) in &mut query {
        explosion.timer.tick(time.delta());

        if explosion.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
