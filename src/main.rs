extern crate piston_window;

mod snake;
mod game;
mod drawing;

use piston_window::*;
use piston_window::types::Color;

use game::Game;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

fn main() {
    // Create a window
    let mut window: PistonWindow = WindowSettings::new("Hello Meow!!",
        [640, 480]).exit_on_esc(true).build().unwrap();

    // Create a snake
    let mut game = Game::new();

    // Event loop
    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // Draw all of them
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
    }
}
