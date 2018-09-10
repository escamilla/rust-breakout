use piston_window::{ellipse, Context, G2d};

use settings;

pub struct Ball {
    radius: u32,
    cx: u32,
    cy: u32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            radius: settings::BALL_RADIUS,
            cx: settings::BALL_X_INITIAL,
            cy: settings::BALL_Y_INITIAL,
        }
    }

    pub fn render(&self, context: &Context, g2d: &mut G2d) {
        ellipse(
            settings::BALL_COLOR,
            [
                self.cx as f64,
                self.cy as f64,
                self.radius as f64,
                self.radius as f64,
            ],
            context.transform,
            g2d,
        );
    }
}
