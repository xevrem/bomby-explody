use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // register events
    app.add_event::<BlastEvent>().add_event::<DamageEvent>();
}

#[derive(Event)]
pub struct BlastEvent {
    pub source: Entity,
    pub location: Vec2,
    pub range: f32,
}

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: i32,
}
