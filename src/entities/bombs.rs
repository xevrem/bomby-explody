use crate::{
    assets::AssetsState, components::*, screens::Screen, AppSystems, GameplaySystems,
    PausableSystems,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<BombAssets>(),
    );
    app.add_event::<BlastEvent>();
    app.add_systems(OnEnter(Screen::Gameplay), add_click_to_spawn_observer);
    app.add_systems(
        Update,
        (bomb_timer_countdown, despawn_explosion_timer)
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
    app.add_systems(
        Update,
        (chain_blast, explode_exploding_bombs)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

pub fn create_bomb(assets: &BombAssets, position: Vec2) -> impl Bundle {
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

#[derive(Event)]
pub struct BlastEvent {
    pub source: Entity,
    pub location: Vec2,
    pub range: f32,
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
            mark_bomb_for_explode(&mut commands, entity);
        }
    }
}

fn mark_bomb_for_explode(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(Exploding);
}

fn explode_exploding_bombs(
    mut commands: Commands,
    assets: Res<BombAssets>,
    mut blast_writer: EventWriter<BlastEvent>,
    mut query: Query<(Entity, &GlobalTransform), (With<Bomb>, With<Exploding>)>,
) {
    for (entity, trans) in &mut query {
        explode_bomb(&mut commands, &assets, &mut blast_writer, entity, &trans);
    }
}

fn explode_bomb(
    commands: &mut Commands,
    assets: &BombAssets,
    blast_writer: &mut EventWriter<BlastEvent>,
    entity: Entity,
    transform: &GlobalTransform,
) {
    // destroy
    commands.entity(entity).despawn_recursive();

    // make splosion
    commands.spawn(create_explosion(
        &assets,
        transform.translation().truncate(),
    ));

    blast_writer.write(BlastEvent {
        source: entity,
        location: transform.translation().truncate(),
        range: 100.0,
    });
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

fn despawn_explosion_timer(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Explosion)>,
    time: Res<Time>,
) {
    for (entity, mut explosion) in &mut query {
        explosion.timer.tick(time.delta());

        if explosion.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn chain_blast(
    mut commands: Commands,
    mut blast_reader: EventReader<BlastEvent>,
    mut bomb_query: Query<(Entity, &GlobalTransform), (With<Bomb>, Without<Exploding>)>,
) {
    if blast_reader.len() > 0 {
        // TODO: process events
        for blast in blast_reader.read() {
            // if let Ok((bomb_ent, bomb_trans)) = bomb_query.get(blast.source) {
            for (bomb_ent, bomb_trans) in &mut bomb_query {
                if bomb_ent == blast.source {
                    // skip if they're the same
                    continue;
                } else if blast.location.distance(bomb_trans.translation().truncate()) < blast.range
                {
                    // other bomb within distance, ASPLODE
                    mark_bomb_for_explode(&mut commands, bomb_ent);
                }
            }
        }
    }
}
