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
        bomb_timer_countdown
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),
    );
}

fn bomb_timer_countdown(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Bomb)>,
    time: Res<Time>,
) {
    for (entity, mut bomb) in &mut query {
        bomb.timer.tick(time.delta());
        if bomb.timer.just_finished() {
            // BOOM
            explode_bomb(&mut commands, entity);
        }
    }
}

fn explode_bomb(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn_recursive();
    // TODO: spawn explosion here
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
    #[asset(path = "images/vfx/Lavaball.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub ball: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 4))]
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
