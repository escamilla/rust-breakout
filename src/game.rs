extern crate piston_window;

use piston_window::{clear, Context, G2d};

use paddle::Paddle;
use settings;

pub struct Game {
    width: u32,
    height: u32,
    paddle: Paddle,
}

impl Game {
    pub fn new() -> Self {
        Game {
            width: settings::GAME_WIDTH,
            height: settings::GAME_HEIGHT,
            paddle: Paddle::new(),
        }
    }

    pub fn render(&self, context: &Context, g2d: &mut G2d) {
        clear(settings::BACKGROUND_COLOR, g2d);
        self.paddle.render(context, g2d);
    }
}
