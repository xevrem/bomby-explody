use bevy::prelude::*;

use crate::constants::SCREEN_WIDTH;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AnimationConfig>()
        .register_type::<Bomb>()
        .register_type::<Explosion>()
        .register_type::<MovementConfig>()
        .register_type::<Spawner>()
        .register_type::<Wave>()
        .register_type::<WillExplode>();
}

#[derive(Component)]
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

#[derive(Component)]
pub struct AssetIdx(pub usize);

#[derive(Component)]
pub struct Attacking;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Blastable;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Bomb {
    pub timer: Timer,
}

#[derive(Component, Clone)]
pub struct Bomber;

#[derive(Component)]
pub struct BombToss {
    pub ease: EasingCurve<Vec2>,
    pub bounce: EasingCurve<f32>,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Countdown {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Damageable;

#[derive(Component)]
pub struct Damaged {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Dead {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Done;

#[derive(Component)]
pub struct EaseFunc<T>(pub EasingCurve<T>);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Exploding;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Explosion {
    pub timer: Timer,
}

#[derive(Component, Clone)]
pub struct Flying;

#[derive(Component, Clone)]
pub struct Ground;

#[derive(Component)]
pub struct Health {
    pub current: i32,
}

#[derive(Component)]
pub struct HealthLabel;

#[derive(Component)]
pub struct Level;

#[derive(Component)]
pub struct LobShot {
    pub height: f32,
    pub timer: Timer,
    pub ease_pos: EasingCurve<Vec2>,
    pub ease_up: EasingCurve<f32>,
    pub ease_down: EasingCurve<f32>,
}

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
        MovementConfig::new(direction, speed) // { direction, speed }
    }

    pub fn with_speed_as_screen_width_percent(mut self, value: f32) -> Self {
        self.speed = SCREEN_WIDTH * value;
        self
    }

    pub fn with_speed(mut self, value: f32) -> Self {
        self.speed = value;
        self
    }

    // pub fn with_speed_as_screen_height_percent(mut self, value: f32) -> Self {
    //     self.speed = SCREEN_HEIGHT * value;
    //     self
    // }
}

#[derive(Component)]
pub struct Moving;

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component)]
pub struct Music;

#[derive(Component)]
pub struct PlaceBombObserver;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerLabel;

#[derive(Component)]
pub struct ScreenWrap;

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component)]
pub struct SoundEffect;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Spawner {
    pub all_spawned: bool,
    pub limit: usize,
    pub max_at_once: usize,
    pub spawned: usize,
    pub timer: Timer,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct SubType<T>(pub T) where T: Component + Clone;

#[derive(Component)]
pub struct TargetDistance(pub f32);

#[derive(Component)]
pub struct TargetPosition {
    pub position: Vec2,
}

#[derive(Component)]
pub struct WasAttacking;

#[derive(Component)]
pub struct WasMoving;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Wave {
    pub level: u32,
    pub limit: usize,
    pub limit_growth: usize,
    pub max_at_once: usize,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WillExplode {
    pub timer: Timer,
}
