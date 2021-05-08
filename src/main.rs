extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;
mod utilities;

use piston_window::*;
use game::Game;
use draw::to_coordinate;
use utilities::{make_color};

const BACK_COLOR : &str  = "ffee93";

fn main() {
    let (w, h) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new("Snake", [to_coordinate(w), to_coordinate(h)])
                                        .exit_on_esc(true)
                                        .build()
                                        .unwrap();

    let mut game = Game::new(w, h);
    
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _|{
            clear(make_color(BACK_COLOR, None), g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
