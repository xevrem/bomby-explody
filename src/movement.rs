use bevy::prelude::*;

use crate::{components::*, AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_movement.in_set(AppSystems::Update).in_set(PausableSystems));
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
