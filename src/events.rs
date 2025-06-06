use bevy::prelude::*;

#[derive(Event)]
pub struct BlastEvent {
    pub source: Entity,
    pub location: Vec2,
    pub range: f32,
}
