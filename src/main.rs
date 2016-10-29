extern crate piston_window;

mod snake;

use piston_window::*;
use piston_window::types::Color;

use snake::{Snake, Direction};

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

fn main() {
    // Create a window
    let mut window: PistonWindow = WindowSettings::new("Hello Meow!!",
        [640, 480]).exit_on_esc(true).build().unwrap();

    // Create a snake
    let mut snake = Snake::new(100.0, 100.0);

    // Event loop
    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => snake.move_forward(Some(Direction::Up)),
                Key::Down => snake.move_forward(Some(Direction::Down)),
                Key::Left => snake.move_forward(Some(Direction::Left)),
                Key::Right => snake.move_forward(Some(Direction::Right)),
                Key::Space => snake.move_forward(None),
                _ => {}
            }
        }

        // Draw all of them
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);

            snake.draw(&c, g);
        });
    }
}
