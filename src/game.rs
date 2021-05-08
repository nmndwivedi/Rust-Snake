use piston_window::*;
use piston_window::types::Color;
use rand::{thread_rng, Rng};
use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};
use crate::utilities::{make_color};

const FOOD_COLOR    : &str  = "fb3640";
const BORDER_COLOR  : &str  = "a0937d"; 
const RESTART_COLOR : &str  = "ff7b54";
const MOVE_TIME     : f64   = 0.15;
const RESTART_TIME  : f64   = 1.0;

pub struct Game {
    snake           : Snake,
    food_exists     : bool,
    food_x          : i32,
    food_y          : i32,
    width           : i32,
    height          : i32,
    game_over       : bool,
    waiting_time    : f64,
    food_color      : Color,
    border_color    : Color,
    restart_color   : Color,
}

impl Game {
    pub fn new(w: i32, h: i32) -> Game {
        
        Game {
            snake: Snake::new(2, 1),
            food_exists: true,
            waiting_time: 0.0,
            food_x: 6,
            food_y: 4,
            width: w,
            height: h,
            game_over: false,
            food_color: make_color(FOOD_COLOR, None),
            border_color: make_color(BORDER_COLOR, None),
            restart_color: make_color(RESTART_COLOR, Some(0.5)),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::U),
            Key::Down => Some(Direction::D),
            Key::Left => Some(Direction::L),
            Key::Right => Some(Direction::R),
            _ => None
        };

        if let Some(dir_input) = dir {
            if dir_input == self.snake.head_direction().opposite() {
                return;
            }
        } else {
            return;
        }

        

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(self.food_color, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(self.border_color, 0, 0, self.width, 1, con, g);
        draw_rectangle(self.border_color, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(self.border_color, 0, 0, 1, self.height, con, g);
        draw_rectangle(self.border_color, self.width - 1,0, 1, self.height, con, g);
    
        if self.game_over {
            draw_rectangle(self.restart_color, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVE_TIME {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_alive(&mut self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return  false;
        };

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1..(self.width - 1));
        let mut new_y = rng.gen_range(1..(self.height - 1));
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..(self.width - 1));
            new_y = rng.gen_range(1..(self.height - 1));
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_alive(dir) {
            self.snake.move_snake(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}