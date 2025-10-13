use crate::{
    assets::AssetsState, components::*, screens::Screen, AppSystems, GameplaySystems,
    PausableSystems,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::LoadGameplay).load_collection::<LobShotAssets>(),
    )
    .add_systems(
        Update,
        (arc_lob_shot)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

#[derive(AssetCollection, Resource)]
pub struct LobShotAssets {
    #[asset(path = "images/vfx/Charge_Fire.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub lob_shot: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 2))]
    pub lob_shot_layout: Handle<TextureAtlasLayout>,
}

pub fn create_lob_shot(
    assets: &LobShotAssets,
    height: f32,
    spawn_pos: Vec2,
    target_pos: Vec2,
) -> impl Bundle {
    (
        Name::new("Lob Shot"),
        LobShot {
            height,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            ease_pos: EasingCurve::new(spawn_pos, target_pos, EaseFunction::Linear),
            ease_up: EasingCurve::new(0.0, 1.0, EaseFunction::CircularOut),
            ease_down: EasingCurve::new(1.0, 0.0, EaseFunction::CircularIn),
        },
        StateScoped(Screen::Gameplay),
        Sprite {
            image: assets.lob_shot.clone(),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: assets.lob_shot_layout.clone(),
            }),
            custom_size: Some(Vec2::splat(96.0)),
            ..default()
        },
        Transform::from_translation(spawn_pos.extend(0.0)),
        AnimationConfig::new(0, 7, 24),
        Animating,
        TargetPosition {
            position: target_pos,
        },
    )
}

fn arc_lob_shot(mut query: Query<(&mut Transform, &mut LobShot)>, time: Res<Time>) {
    // for (mut transform, config) in &mut query {
    //     let unit_rate = time.delta_secs() * config.speed;
    //     let delta = unit_rate * config.direction;
    //     transform.translation += delta.extend(0.0);
    // }

    for (mut transform, lob_shot) in &mut query {
        let fraction = lob_shot.timer.fraction();
        let mut new_pos = lob_shot.ease_pos.sample_clamped(fraction);

        if fraction < 0.5 {
            let up_frac = fraction / 0.5;
            // going up
            let up = lob_shot.ease_up.sample_clamped(up_frac) * lob_shot.height;
            new_pos.y += up;
        } else {
            // going down
            let down_frac = (fraction - 0.5) / 0.5;
            // going up
            let down = lob_shot.ease_down.sample_clamped(down_frac) * lob_shot.height;
            new_pos.y += down;
        }

        transform.translation = new_pos.extend(0.0);
    }
}
