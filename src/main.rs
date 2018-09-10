extern crate piston_window;

use piston_window::{Button, PistonWindow, PressEvent, ReleaseEvent, WindowSettings};

use game::Game;

mod ball;
mod game;
mod paddle;
mod settings;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Breakout", [settings::GAME_WIDTH, settings::GAME_HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut game = Game::new();
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            game.key_released(key);
        }

        window.draw_2d(&event, |context, graphics| {
            game.render(&context, graphics);
            game.update();
        });
    }
}
