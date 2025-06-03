use bevy::prelude::*;

use crate::components::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, tick_spawners);
}

pub fn create_enemy_spawner(mut commands: Commands) {
    commands.spawn((Name::new("Enemy Spawner"), Enemy, Spawner));
}

fn tick_spawners() {
    //
}
