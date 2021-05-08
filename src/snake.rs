use std::collections::LinkedList;
use piston_window::{Context, G2d};

use crate::draw::draw_block;
use crate::utilities::{make_color};

const SNAKE_COLOR: &str = "83a95c";

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    U,
    D,
    L,
    R
}

impl Direction {
    pub fn opposite(&self) -> Direction{
        match *self {
            Direction::U => Direction::D,
            Direction::D => Direction::U,
            Direction::L => Direction::R,
            Direction::R => Direction::L,
        }
    }
}

#[derive(Clone, Debug)]
struct Block {
    x: i32,
    y: i32
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block { x: x + 2, y: y });
        body.push_back(Block { x: x + 1, y: y });
        body.push_back(Block { x: x + 0, y: y });

        Snake {
            direction: Direction::R,
            body,
            tail: None
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(make_color(SNAKE_COLOR, None), block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_snake(&mut self, direction: Option<Direction>) {
        match direction {
            Some(d) => self.direction = d,
            None => ()
        }

        let (last_x, last_y) = self.head_position();

        let new_block = match self.direction {
            Direction::U => Block { x: last_x, y: last_y - 1 },
            Direction::D => Block { x: last_x, y: last_y + 1 },
            Direction::L => Block { x: last_x - 1, y: last_y },
            Direction::R => Block { x: last_x + 1, y: last_y },
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();

        let mut moving_dir = self.direction;

        match dir {
            Some(d) => moving_dir = d,
            None => ()
        }
        
        match moving_dir {
            Direction::U => ( head_x, head_y - 1 ),
            Direction::D => ( head_x, head_y + 1 ),
            Direction::L => ( head_x - 1, head_y ),
            Direction::R => ( head_x + 1, head_y ),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();

        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;

        for block in &self.body {
            if x == block.x && y == block.y { return true; }

            ch += 1;

            if ch == self.body.len() - 1 {
                break;
            }
        }

        false
    }
}