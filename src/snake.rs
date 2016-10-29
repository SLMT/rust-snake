
use piston_window::Context;
use piston_window::G2d;
use piston_window::rectangle;
use piston_window::types::Color;

const SNAKE_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

const BLOCK_SIZE: f64 = 50.0;

enum Direction {
    Up, Down, Left, Right
}

struct Block {
    x: f64,
    y: f64
}

pub struct Snake {
    moving_direction: Direction,
    body: Vec<Block>
}

impl Snake {
    pub fn new(init_x: f64, init_y: f64) -> Snake {
        Snake {
            moving_direction: Direction::Right,
            body: vec![
                Block {
                    x: init_x,
                    y: init_y
                },
                Block {
                    x: init_x + BLOCK_SIZE,
                    y: init_y
                },
                Block {
                    x: init_x + BLOCK_SIZE * 2.0,
                    y: init_y
                }
            ]
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            rectangle(SNAKE_COLOR, [block.x, block.y,
                BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
        }
    }
}
