extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};

use game::Game;

mod game;
mod paddle;
mod settings;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Breakout", [settings::GAME_WIDTH, settings::GAME_HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let game = Game::new();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            game.render(&context, graphics);
        });
    }
}
