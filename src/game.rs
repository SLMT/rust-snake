
use piston_window::*;
use piston_window::types::Color;

use snake::{Snake, Direction};
use drawing::{draw_block, draw_rectange};

const FOOD_COLOR: Color = [0.90, 0.49, 0.13, 1.0];
const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

pub struct Game {
    snake: Snake,

    // Food
    food_exist: bool,
    food_x: i32,
    food_y: i32,

    // Border
    width: i32,
    height: i32
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exist: true,
            food_x: 5,
            food_y: 3,
            width: width,
            height: height
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None
        };

        // Check the next position of the snake
        if self.check_next_position(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        // Draw the food
        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Draw the border
        draw_rectange(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectange(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectange(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectange(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_last_removed();
        }
    }

    fn check_next_position(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head_position(dir);

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }
}
