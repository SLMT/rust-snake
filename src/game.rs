
use piston_window::*;
use piston_window::types::Color;

use snake::{Snake, Direction};
use drawing::draw_block;

const FOOD_COLOR: Color = [0.90, 0.49, 0.13, 1.0];

pub struct Game {
    snake: Snake,

    // Food
    food_exist: bool,
    food_x: i32,
    food_y: i32
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exist: true,
            food_x: 5,
            food_y: 3
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Up => self.snake.move_forward(Some(Direction::Up)),
            Key::Down => self.snake.move_forward(Some(Direction::Down)),
            Key::Left => self.snake.move_forward(Some(Direction::Left)),
            Key::Right => self.snake.move_forward(Some(Direction::Right)),
            Key::Space => self.snake.move_forward(None),
            _ => {}
        }

        self.check_eating();
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        // Draw the food
        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_last_removed();
        }
    }
}
