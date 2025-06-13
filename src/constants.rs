pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const SCREEN_HALF_WIDTH: f32 = 1280.0 / 2.0;
pub const SCREEN_HALF_HEIGHT: f32 = 720.0 / 2.0;

pub const BASE_TILE_SIZE: f32 = 16.0;
pub const TILE_SIZE: f32 = BASE_TILE_SIZE * 2.0;
pub const NUM_TILES_X: i32 = (SCREEN_WIDTH / TILE_SIZE) as i32;
pub const NUM_TILES_Y: i32 = (SCREEN_HEIGHT / TILE_SIZE) as i32;
