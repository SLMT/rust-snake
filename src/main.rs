extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];
const SNAKE_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Meow!!",
        [640, 480]).exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);

            rectangle(SNAKE_COLOR, [100.0, 100.0, 50.0, 50.0],
                c.transform, g);
        });
    }
}
