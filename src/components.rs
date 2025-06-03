use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct AnimationConfig {
    pub index: usize,
    pub frames: usize,
    pub timer: Timer,
    pub fps: usize,
}

impl AnimationConfig {
    pub fn new(index: usize, frames: usize, fps: usize) -> Self {
        Self {
            index,
            frames,
            fps,
            timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: usize) -> Timer {
        Timer::from_seconds(1.0 / fps as f32, TimerMode::Once)
    }

    pub fn timer_from_self_fps(&self) -> Timer {
        Timer::from_seconds(1.0 / self.fps as f32, TimerMode::Once)
    }
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
