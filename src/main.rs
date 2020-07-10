extern crate piston_window;

mod draw;

use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

use crate::draw::draw_window;

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", draw_window(width, height))
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear(BACK_COLOR, graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
}
