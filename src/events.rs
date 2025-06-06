use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // register events
    app.add_event::<BlastEvent>();
}

#[derive(Event)]
pub struct BlastEvent {
    pub source: Entity,
    pub location: Vec2,
    pub range: f32,
}
