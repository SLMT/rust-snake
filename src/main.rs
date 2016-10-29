extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Meow!!",
        [640, 480]).exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        
    }
}
