extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const SNAKE_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

struct Point<T> {
    x: T, y: T
}

fn main() {
    // Create a window
    let mut window: PistonWindow = WindowSettings::new("Hello Meow!!",
        [640, 480]).exit_on_esc(true).build().unwrap();

    let mut snake_pos: Point<f64> =
        Point {x: 200.0, y: 200.0};

    // Event loop
    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => snake_pos.y -= 10.0,
                Key::Down => snake_pos.y += 10.0,
                Key::Left => snake_pos.x -= 10.0,
                Key::Right => snake_pos.x += 10.0,
                _ => {}
            }
        }

        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);

            rectangle(SNAKE_COLOR, [snake_pos.x, snake_pos.y,
                50.0, 50.0], c.transform, g);
        });
    }
}
