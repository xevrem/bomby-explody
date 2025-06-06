use bevy::{prelude::*, window::PrimaryWindow};

use crate::{components::*, AppSystems, GameplaySystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (apply_movement, apply_screen_wrap)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(GameplaySystems),
    );
}

fn apply_movement(
    mut query: Query<(&mut Transform, &MovementConfig), With<Moving>>,
    time: Res<Time>,
) {
    for (mut transform, config) in &mut query {
        let unit_rate = time.delta_secs() * config.speed;
        let delta = unit_rate * config.direction;
        transform.translation += delta.extend(0.0);
    }
}


fn apply_screen_wrap(
    window: Single<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    let size = window.size() + 256.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}
