use crate::{
    assets::AssetsState,
    audio::{sound_effect, SfxAssets},
    components::*,
    screens::Screen,
    vfx::{explosion::create_explosion_vfx, VfxAssets},
    AppSystems, GameplaySystems, PausableSystems,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<BombAssets>(),
    );
    app.add_event::<BlastEvent>();
    app.add_systems(OnEnter(Screen::Gameplay), add_click_to_spawn_observer);
    app.add_systems(
        Update,
        (bomb_timer_countdown, countdown_to_exploding)
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

#[derive(AssetCollection, Resource)]
pub struct BombAssets {
    #[asset(path = "images/vfx/Charge_Fire.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub charge: Handle<Image>,
    #[asset(path = "images/vfx/Lavaball.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub ball: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 2))]
    pub ball_layout: Handle<TextureAtlasLayout>,
}

#[derive(Event)]
pub struct BlastEvent {
    pub source: Entity,
    pub location: Vec2,
    pub range: f32,
}

pub fn create_bomb(assets: &BombAssets, position: Vec2, timeout: f32) -> impl Bundle {
    (
        Name::new("Bomb"),
        Bomb {
            timer: Timer::from_seconds(timeout, TimerMode::Once),
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
            commands.spawn(create_bomb(&assets, location, 2.0));
        }
    }
}

fn bomb_timer_countdown(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Bomb), (Without<Exploding>, Without<WillExplode>)>,
    time: Res<Time>,
) {
    for (entity, mut bomb) in &mut query {
        bomb.timer.tick(time.delta());
        if bomb.timer.just_finished() {
            // BOOM
            mark_bomb_for_explode(&mut commands, entity, 1.0);
        }
    }
}

fn mark_bomb_for_explode(commands: &mut Commands, entity: Entity, timeout: f32) {
    commands.entity(entity).insert(WillExplode {
        timer: Timer::from_seconds(timeout, TimerMode::Once),
    });
}

fn explode_exploding_bombs(
    mut commands: Commands,
    assets: Res<VfxAssets>,
    sfx: Res<SfxAssets>,
    mut blast_writer: EventWriter<BlastEvent>,
    mut query: Query<(Entity, &GlobalTransform), (With<Bomb>, With<Exploding>)>,
    mut entropy: GlobalEntropy<WyRand>,
) {
    for (entity, trans) in &mut query {
        explode_bomb(
            &mut commands,
            &assets,
            &sfx,
            &mut blast_writer,
            entity,
            &trans,
            &mut entropy,
        );
    }
}

fn explode_bomb(
    commands: &mut Commands,
    assets: &VfxAssets,
    sfx: &SfxAssets,
    blast_writer: &mut EventWriter<BlastEvent>,
    entity: Entity,
    transform: &GlobalTransform,
    entropy: &mut GlobalEntropy<WyRand>,
) {
    // destroy
    commands.entity(entity).despawn();

    // make splosion
    commands.spawn(create_explosion_vfx(
        &assets,
        transform.translation().truncate(),
    ));

    if let Some(random_step) = sfx.bombs.choose(entropy.as_mut()) {
        commands.spawn(sound_effect(random_step.clone(), 0.15));
    } else {
        warn!("no bomb sound :(");
    }

    blast_writer.write(BlastEvent {
        source: entity,
        location: transform.translation().truncate(),
        range: 100.0,
    });
}

fn chain_blast(
    mut commands: Commands,
    mut blast_reader: EventReader<BlastEvent>,
    mut bomb_query: Query<
        (Entity, &GlobalTransform),
        (With<Bomb>, Without<Exploding>, Without<WillExplode>),
    >,
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
                    mark_bomb_for_explode(&mut commands, bomb_ent, 0.1);
                }
            }
        }
    }
}

fn countdown_to_exploding(
    mut commands: Commands,
    mut query: Query<(Entity, &mut WillExplode), (With<Bomb>, Without<Exploding>)>,
    time: Res<Time>,
) {
    for (entity, mut will_explode) in &mut query {
        will_explode.timer.tick(time.delta());
        if will_explode.timer.just_finished() {
            // BOOM
            commands.entity(entity).insert(Exploding);
        }
    }
}
