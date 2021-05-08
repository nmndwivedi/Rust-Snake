use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 20.0;

pub fn to_coordinate(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coordinate(x);
    let gui_y = to_coordinate(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    )
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, w: i32, h: i32, con: &Context, g: &mut G2d) {
    let x = to_coordinate(x);
    let y = to_coordinate(y);

    rectangle(
        color,
        [x, y, BLOCK_SIZE * (w as f64), BLOCK_SIZE * (h as f64)],
        con.transform,
        g
    );
}