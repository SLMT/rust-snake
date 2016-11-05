
use std::collections::LinkedList;

use piston_window::Context;
use piston_window::G2d;
use piston_window::types::Color;

use drawing::draw_block;

const SNAKE_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

pub enum Direction {
    Up, Down, Left, Right
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32
}

pub struct Snake {
    moving_direction: Direction,
    body: LinkedList<Block>,
    last_removed_block: Option<Block>
}

impl Snake {
    pub fn new(init_x: i32, init_y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: init_x + 2,
            y: init_y
        });
        body.push_back(Block {
            x: init_x + 1,
            y: init_y
        });
        body.push_back(Block {
            x: init_x,
            y: init_y
        });

        Snake {
            moving_direction: Direction::Right,
            body: body,
            last_removed_block: None
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // Change moving direction
        match dir {
            Some(d) => self.moving_direction = d,
            None => {}
        }

        // Retrieve the position of the head block
        let (last_x, last_y): (i32, i32) = self.head_position();

        // The snake moves
        let new_block = match self.moving_direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y
            }
        };
        self.body.push_front(new_block);
        let removed_blk = self.body.pop_back().unwrap();
        self.last_removed_block = Some(removed_blk);
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn restore_last_removed(&mut self) {
        let blk = self.last_removed_block.clone().unwrap();
        self.body.push_back(blk);
    }
}
