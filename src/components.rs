use bevy::prelude::*;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct Animating;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct AnimationConfig {
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
pub struct Bomb {
    pub timer: Timer,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Exploding;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Explosion {
    pub timer: Timer,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementConfig {
    /// direction of movement
    pub direction: Vec2,
    /// units per second
    pub speed: f32,
}

impl MovementConfig {
    pub fn new(direction: Vec2, speed: f32) -> Self {
        Self { direction, speed }
    }

    pub fn from_vec2(vec: Vec2) -> Self {
        let speed = vec.length();
        let direction = vec.normalize();
        Self { direction, speed }
    }

    pub fn with_speed_as_screen_width_percent(mut self, value: f32) -> Self {
        self.speed = SCREEN_WIDTH * value;
        self
    }

    pub fn with_speed_as_screen_height_percent(mut self, value: f32) -> Self {
        self.speed = SCREEN_HEIGHT * value;
        self
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Moving;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Spawner;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AnimationConfig>();
    app.register_type::<MovementConfig>();
}
