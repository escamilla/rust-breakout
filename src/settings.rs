use piston_window::types::Color;

pub const GAME_WIDTH: u32 = 640;
pub const GAME_HEIGHT: u32 = 480;

pub const PADDLE_WIDTH: u32 = ((GAME_WIDTH as f64) * 0.2) as u32;
pub const PADDLE_HEIGHT: u32 = ((GAME_HEIGHT as f64) * 0.05) as u32;
pub const PADDLE_X_INITIAL: u32 = (((GAME_WIDTH as f64) * 0.5) as u32) - (PADDLE_WIDTH / 2);
pub const PADDLE_Y: u32 = (((GAME_HEIGHT as f64) * 0.9) as u32) - (PADDLE_HEIGHT / 2);
pub const PADDLE_X_DELTA: u32 = ((GAME_WIDTH as f64) * 0.01) as u32;

pub const BALL_RADIUS: u32 = ((PADDLE_WIDTH as f64) * 0.1) as u32;
pub const BALL_CX_INITIAL: i32 = (GAME_WIDTH / 2) as i32;
pub const BALL_CY_INITIAL: i32 = (GAME_HEIGHT / 2) as i32;
pub const BALL_DX: i32 = ((BALL_RADIUS as f64) * 0.25) as i32;
pub const BALL_DY: i32 = ((BALL_RADIUS as f64) * 0.25) as i32;

pub const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const PADDLE_COLOR: Color = [0.0, 0.0, 1.0, 1.0];
pub const BALL_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
