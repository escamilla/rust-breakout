use piston_window::{ellipse, Context, G2d};

use settings;

pub struct Ball {
    radius: u32,
    cx: i32,
    cy: i32,
    dx: i32,
    dy: i32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            radius: settings::BALL_RADIUS,
            cx: settings::BALL_CX_INITIAL,
            cy: settings::BALL_CY_INITIAL,
            dx: settings::BALL_DX,
            dy: settings::BALL_DY,
        }
    }

    pub fn render(&self, context: &Context, g2d: &mut G2d) {
        ellipse(
            settings::BALL_COLOR,
            [
                (self.cx - (self.radius as i32)) as f64,
                (self.cy - (self.radius as i32)) as f64,
                (self.radius * 2) as f64,
                (self.radius * 2) as f64,
            ],
            context.transform,
            g2d,
        );
    }

    pub fn update(&mut self) {
        self.cx += self.dx;
        if self.cx < 0 {
            self.cx = 0;
            self.dx *= -1;
        } else if self.cx >= (settings::GAME_WIDTH as i32) {
            self.cx = (settings::GAME_WIDTH as i32) - 1;
            self.dx *= -1;
        }

        self.cy += self.dy;
        if self.cy < 0 {
            self.cy = 0;
            self.dy *= -1;
        } else if self.cy >= (settings::GAME_HEIGHT as i32) {
            self.cy = (settings::GAME_HEIGHT as i32) - 1;
            self.dy *= -1;
        }
    }
}
