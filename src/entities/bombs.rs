use crate::{
    assets::AssetsState,
    audio::{sound_effect, SfxAssets},
    components::*,
    constants::SCREEN_HALF_WIDTH,
    events::BlastEvent,
    menus::Menu,
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
    app.add_systems(OnEnter(Menu::None), add_click_to_spawn_observer);
    app.add_systems(
        Update,
        (bomb_timer_countdown, countdown_to_exploding)
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
    app.add_systems(
        Update,
        chain_blast
            .in_set(AppSystems::Events)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
    app.add_systems(
        Update,
        (explode_exploding_bombs, move_towards_target)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

#[derive(AssetCollection, Resource)]
pub struct BombAssets {
    #[asset(path = "images/vfx/Lavaball.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub ball: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 2))]
    pub ball_layout: Handle<TextureAtlasLayout>,
}

pub fn create_bomb(
    assets: &BombAssets,
    position: Vec2,
    timeout: f32,
    speed: f32,
    player_pos: Vec3,
) -> impl Bundle {
    let start_pos = player_pos + Vec3::new(24.0, 0.0, 0.0);
    let distance = start_pos.xy().distance(position);
    let lerp_time = (distance / speed) / 2.0;
    (
        Name::new("Bomb"),
        AnimationConfig::new(0, 8, 6),
        Animating,
        Bomb {
            timer: Timer::from_seconds(timeout, TimerMode::Once),
        },
        BombToss {
            height: 100.0,
            ease: EasingCurve::new(start_pos.xy(), position, EaseFunction::Linear),
            bounce_up: EasingCurve::new(0.0, 1.0, EaseFunction::CircularOut),
            bounce_down: EasingCurve::new(1.0, 0.0, EaseFunction::BounceOut),
        },
        // for target position lerp
        Countdown {
            timer: Timer::from_seconds(lerp_time, TimerMode::Once),
        },
        StateScoped(Screen::Gameplay),
        Sprite {
            image: assets.ball.clone(),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: assets.ball_layout.clone(),
                // ..default()
            }),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        TargetPosition { position },
        Transform::from_translation(start_pos),
    )
}

fn add_click_to_spawn_observer(
    mut commands: Commands,
    prior: Option<Single<Entity, With<PlaceBombObserver>>>,
) {
    // if no observer exists, add it
    if prior.is_none() {
        commands.spawn((
            // auto destroy observer:
            //   if we open a menu
            //   or leave the gameplay screen
            StateScoped(Screen::Gameplay),
            StateScoped(Menu::None),
            Observer::new(place_bomb_on_click),
            PlaceBombObserver,
        ));
    }
}

fn place_bomb_on_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    assets: Res<BombAssets>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    player_query: Single<&Transform, With<Player>>,
) {
    let (camera, camera_trans) = *camera_query;
    if let Ok(location) =
        camera.viewport_to_world_2d(camera_trans, trigger.pointer_location.position)
    {
        // info!("clicked here: {}", location);
        commands.spawn(create_bomb(
            &assets,
            location,
            2.75,
            200.0,
            player_query.translation,
        ));
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
            mark_bomb_for_explode(&mut commands, entity, 0.25);
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
    mut exploding_bomb_query: Query<(Entity, &GlobalTransform), (With<Bomb>, With<Exploding>)>,
    mut entropy: GlobalEntropy<WyRand>,
) {
    let count = exploding_bomb_query.iter().len();
    for (entity, trans) in &mut exploding_bomb_query {
        explode_bomb(
            &mut commands,
            &assets,
            &sfx,
            &mut blast_writer,
            entity,
            trans,
            &mut entropy,
            count,
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
    bomb_count: usize,
) {
    // destroy
    commands.entity(entity).despawn();

    // make splosion
    commands.spawn(create_explosion_vfx(
        assets,
        transform.translation().truncate(),
    ));

    if let Some(random_step) = sfx.bombs.choose(entropy.as_mut())
        && bomb_count < 5
    {
        commands.spawn(sound_effect(random_step.clone(), 0.15));
    }

    blast_writer.write(BlastEvent {
        source: entity,
        location: transform.translation().xy(),
        range: 50.0,
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
    if !blast_reader.is_empty() {
        for blast in blast_reader.read() {
            for (bomb_ent, bomb_trans) in &mut bomb_query {
                // skip if they're the same
                if bomb_ent == blast.source {
                    continue;
                }
                // other bomb within distance, ASPLODE
                else if blast.location.distance(bomb_trans.translation().truncate()) < blast.range
                {
                    mark_bomb_for_explode(&mut commands, bomb_ent, 0.25);
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

fn move_towards_target(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &TargetPosition,
            &mut Countdown,
            &BombToss,
        ),
        With<Bomb>,
    >,
    time: Res<Time>,
) {
    for (entity, mut trans, target_pos, mut countdown, bomb_toss) in &mut query {
        countdown.timer.tick(time.delta());
        if countdown.timer.just_finished() {
            commands.entity(entity).remove::<TargetPosition>();
            mark_bomb_for_explode(&mut commands, entity, 0.25);
        } else {
            let fraction = countdown.timer.fraction();
            let mut new_pos = bomb_toss.ease.sample_clamped(fraction);

            if fraction < 0.25 {
                let up_frac = fraction / 0.25;
                new_pos.y += bomb_toss.bounce_up.sample_clamped(up_frac) * bomb_toss.height;
            } else {
                let down_frac = (fraction - 0.25) / 0.75;
                new_pos.y += bomb_toss.bounce_down.sample_clamped(down_frac) * bomb_toss.height;
            }

            trans.translation = new_pos.extend(0.0);
        }
    }
}
