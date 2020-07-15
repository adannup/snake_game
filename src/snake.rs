use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Debug)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    body: LinkedList<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake { body }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            println!("block: {:?}", block);
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }
}
