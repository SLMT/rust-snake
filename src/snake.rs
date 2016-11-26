
use std::collections::LinkedList;

use piston_window::Context;
use piston_window::G2d;
use piston_window::types::Color;

use drawing::draw_block;

const SNAKE_COLOR: Color = [0.18, 0.80, 0.44, 1.0];

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
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

    pub fn head_direction(&self) -> Direction {
        self.moving_direction
    }

    pub fn next_head_position(&self, dir: Option<Direction>) -> (i32, i32) {
        // Retrieve the position of the head block
        let (head_x, head_y): (i32, i32) = self.head_position();

        // Get moving direction
        let mut moving_dir = self.moving_direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        // The snake moves
        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y)
        }
    }

    pub fn restore_last_removed(&mut self) {
        let blk = self.last_removed_block.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn is_overlap_except_tail(&self, x: i32, y: i32) -> bool {
        let mut checked = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            // For excluding the tail
            checked += 1;
            if checked == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}
