use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct AnimationConfig {
    pub index: usize,
    pub frames: usize,
    pub timer: Timer,
    pub fps: usize,
}


#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Enemy;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct FrameData {
    pub index: usize,
    pub frames: u32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Spawner;
