extern crate piston_window;

use piston_window::{rectangle, Context, G2d};

use settings;

pub struct Paddle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Paddle {
    pub fn new() -> Self {
        Paddle {
            x: settings::PADDLE_X_INITIAL,
            y: settings::PADDLE_Y,
            width: settings::PADDLE_WIDTH,
            height: settings::PADDLE_HEIGHT,
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
}
