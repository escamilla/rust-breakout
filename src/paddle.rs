extern crate piston_window;

use piston_window::{rectangle, Context, G2d};

use settings;

pub struct Paddle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    direction: Option<Direction>,
}

pub enum Direction {
    Left,
    Right,
}

impl Paddle {
    pub fn new() -> Self {
        Paddle {
            x: settings::PADDLE_X_INITIAL,
            y: settings::PADDLE_Y,
            width: settings::PADDLE_WIDTH,
            height: settings::PADDLE_HEIGHT,
            direction: None,
        }
    }

    pub fn render(&self, context: &Context, g2d: &mut G2d) {
        rectangle(
            settings::PADDLE_COLOR,
            [
                self.x as f64,
                self.y as f64,
                self.width as f64,
                self.height as f64,
            ],
            context.transform,
            g2d,
        );
    }

    pub fn update(&mut self) {
        match self.direction {
            Some(Direction::Left) => {
                if settings::PADDLE_X_DELTA > self.x {
                    self.x = 0;
                } else {
                    self.x -= settings::PADDLE_X_DELTA;
                }
            }
            Some(Direction::Right) => {
                if settings::PADDLE_X_DELTA > (settings::GAME_WIDTH - (self.x + self.width)) {
                    self.x = settings::GAME_WIDTH - self.width;
                } else {
                    self.x += settings::PADDLE_X_DELTA;
                }
            }
            _ => (),
        }
    }

    pub fn start_moving(&mut self, direction: Direction) {
        self.direction = Some(direction);
    }

    pub fn stop_moving(&mut self) {
        self.direction = None;
    }
}
