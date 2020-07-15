use piston_window::types::Color;
use piston_window::*;

use crate::draw::draw_rectangle;
use crate::snake::Snake;

/*
** Constanst
**/
const BORDER_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

pub struct Game {
    snake: Snake,
    width: i32,
    height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            width,
            height,
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
    }
}
