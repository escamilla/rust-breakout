extern crate piston_window;

use piston_window::{clear, Context, G2d, Key};

use ball::Ball;
use paddle::{Direction, Paddle};
use settings;

pub struct Game {
    paddle: Paddle,
    ball: Ball,
}

impl Game {
    pub fn new() -> Self {
        Game {
            paddle: Paddle::new(),
            ball: Ball::new(),
        }
    }

    pub fn render(&self, context: &Context, g2d: &mut G2d) {
        clear(settings::BACKGROUND_COLOR, g2d);
        self.paddle.render(context, g2d);
        self.ball.render(context, g2d);
    }

    pub fn update(&mut self) {
        self.paddle.update();
        self.ball.update();
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Left => self.paddle.start_moving(Direction::Left),
            Key::Right => self.paddle.start_moving(Direction::Right),
            _ => (),
        }
    }

    pub fn key_released(&mut self, key: Key) {
        match key {
            Key::Left | Key::Right => self.paddle.stop_moving(),
            _ => (),
        }
    }
}
