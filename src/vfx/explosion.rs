use crate::{components::*, screens::Screen, AppSystems, GameplaySystems, PausableSystems};
use bevy::prelude::*;

use super::VfxAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        despawn_explosion_timer
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

pub fn create_explosion_vfx(assets: &VfxAssets, location: Vec2) -> impl Bundle {
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
            commands.entity(entity).despawn();
        }
    }
}
