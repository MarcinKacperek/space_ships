pub const ARENA_WIDTH: f32 = 750.0;//375.0;
pub const ARENA_HEIGHT: f32 = 900.0;//450.0;

pub const PLAYER_SHIP_WIDTH: f32 = 32.0;
pub const PLAYER_SHIP_HEIGHT: f32 = 32.0;

pub const MISSILE_WIDTH: f32 = 1.0;
pub const MISSILE_HEIGHT: f32 = 8.0;

pub const ENEMY_WIDTH: f32 = 16.0;
pub const ENEMY_HEIGHT: f32 = 16.0;

pub const ENEMY_SPAWNER_MIN_DELAY: f64 = 0.5;
pub const ENEMY_SPAWNER_MAX_DELAY: f64 = 2.0;

pub const ENEMY_SPAWNER_POINT_MIN_X: f32 = ENEMY_WIDTH / 2.0;
pub const ENEMY_SPAWNER_POINT_MAX_X: f32 = ARENA_WIDTH - ENEMY_WIDTH / 2.0;
pub const ENEMY_SPAWNER_POINT_Y: f32 = ARENA_HEIGHT + 1.0;

pub const UI_FONT_COLOR: [f32; 4] = [0.95, 0.95, 0.95, 1.0];
pub const UI_BUTTON_WIDTH: f32 = 256.0;
pub const UI_BUTTON_HEIGHT: f32 = 64.0;
pub const UI_BUTTON_FONT_SIZE: f32 = 32.0;
pub const UI_GAMEPLAY_FONT_SIZE: f32 = 24.0;