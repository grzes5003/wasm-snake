use wasm_bindgen::prelude::*;
use crate::snake::cell::Cell;
use crate::snake::snake::Snake;
use crate::snake::snake::Direction;
use super::log;
use std::fmt;
use crate::utils;
use self::js_sys::Math::random;

//use js_sys::Date::{new_0, get_time};
extern crate js_sys;

#[wasm_bindgen]
pub struct Universe {
    lost: bool,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    snake: Snake,
    background: String,
    apple: u32,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        //log(format!("tik {}", self.cells.iter().filter(|i| **i == Cell::Alive).count()).as_str());

        // snake ate apple in this particular tick state handle
        let mut ate_apple = false;

        if self.lost {
            // player lost
            //TODO tmp change
            //return;
        }

        let mut cells_vec = self.cells.clone();
        let mut body_vec = self.snake.body.clone();

        //log(format!("-----body vec: {:?}", self.snake.body).as_str());

        // get index of first element
        let mut last = *self.snake.body.get(0).unwrap();

        for index in 0..body_vec.len() {
            //log(format!("index for :::: {}", body_vec[index]).as_str());

            // first element from vec
            if index == 0 as usize {
                let (x, y) = self.get_position(body_vec[index] as usize);
                let current = match self.snake.direction {
                    Direction::Up => {
                        let (x_alt, y_alt) = self.out_of_bounds(x as i32, y as i32 - 1);
                        self.get_index(x_alt, y_alt) as u32
                    }
                    Direction::Down => {
                        //log(format!("PRE {} {}", x, y).as_str());
                        let (x_alt, y_alt) = self.out_of_bounds(x as i32, y as i32 + 1);
                        //log(format!("DOWN {} {}", x_alt, y_alt).as_str());
                        self.get_index(x_alt, y_alt) as u32
                    }
                    Direction::Left => {
                        let (x_alt, y_alt) = self.out_of_bounds(x as i32 - 1, y as i32);
                        self.get_index(x_alt, y_alt) as u32
                    }
                    Direction::Right => {
                        let (x_alt, y_alt) = self.out_of_bounds(x as i32 + 1, y as i32);
                        self.get_index(x_alt, y_alt) as u32
                    }
                };
                cells_vec[current as usize] = Cell::Alive;
                //log(format!("pree cells {:?}", body_vec[index]).as_str());
                body_vec[index] = current;

                if current == self.apple {
                    ate_apple = true;
                }
            } else {
                // more elements from vec
                let tmp_last = body_vec[index];

                body_vec[index] = last;
                //body_element = &last.clone();

                cells_vec[last as usize] = Cell::Alive;
                last = tmp_last;
            }
        }

        // check if snake ate apple
        if ate_apple {
            // append snake
            body_vec.push(last);
            self.spawn_apple();

            cells_vec[self.apple as usize] = Cell::Apple;
        } else {
            // kill last cell in the body
            cells_vec[last as usize] = Cell::Transparent;
        }

        // update snake body
        self.snake.body = body_vec;

        // update universe
        self.cells = cells_vec;

        // clear on same frame when player lost
        if self.lost {
            //TODO tmp change
            //self.clear_universe();
        }

        //log("--------=========---------");
    }

    fn clear_universe(&mut self) {
        let mut cells_cpy = self.cells.clone();
        (0..cells_cpy.len()).into_iter().for_each(|i| cells_cpy[i] = Cell::Dead);
        self.cells = cells_cpy;
        //log(format!("Cleared items {:?}", self.cells.iter().filter(|i| **i == Cell::Alive).count()).as_str());
    }

    fn player_lost(&mut self) {
        self.lost = true;
        self.snake = Snake::new();
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (column * self.width + row) as usize
    }

    fn get_position(&self, index: usize) -> (u32, u32) {
        //log(format!("get position {} {} {} and width {}", index, index as u32 % self.width , index as u32 / self.width, self.width).as_str());
        (index as u32 % self.width, index as u32 / self.width)
    }

    // checks if players hits wall
    fn out_of_bounds(&mut self, x: i32, y: i32) -> (u32, u32) {
        if 0 > x || x >= self.width as i32 {
            //log(format!("LOST UNO {}", x).as_str());
            // lost
            self.player_lost();
            return (0, 0);
        }

        if 0 > y || y >= self.height as i32 {
            //log(format!("LOST duo").as_str());
            // lost
            self.player_lost();
            return (0, 0);
        }

        (x as u32, y as u32)
    }

    fn spawn_apple(&mut self) {
        let range = self.background.len() as i64;
        //let now = js_sys::Date::now();
        //self.apple = (now as i64 % range) as u32;
        let num = (js_sys::Math::random() * range as f64 ) as u32;
        self.apple = num;

        log(format!("apple is {}", self.apple).as_str());
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    // controlls
    pub fn move_right(&mut self) {
        self.snake.direction = Direction::Right;
    }

    pub fn move_left(&mut self) {
        self.snake.direction = Direction::Left;
    }

    pub fn move_up(&mut self) {
        self.snake.direction = Direction::Up;
    }

    pub fn move_down(&mut self) {
        self.snake.direction = Direction::Down;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 32;
        let snake = Snake::new();
        let lost = false;
        let background = String::from("");

        let cells = (0..width * height)
            .map(|i| {
                if i == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            lost,
            width,
            height,
            cells,
            snake,
            background,
            apple: 5,
        }
    }

    pub fn new_with_bcg(s: String) -> Universe {
        utils::set_panic_hook();

        let mut background = s.clone();
        background = background.replace(" ", ".");

        let len = match s.lines().map(|i| i.len()).max() {
            Some(val) => val,
            None => 0 as usize
        };

        let mut back_lines: Vec<_> = background.lines().map(|i| i.to_owned()).collect();
        for index in 0..back_lines.len() {
            back_lines[index] = format!("{}{}", back_lines[index], std::iter::repeat(".").take({ if len - back_lines[index].len() > 0 { len - back_lines[index].len() } else { 0 as usize } }).collect::<String>());
        }

        let width = len as u32;
        let height = back_lines.len() as u32;
        let snake = Snake::new();
        let cells =
            (0..width * height).map(|i| {
                if i == 5 {
                    Cell::Apple
                } else {
                    Cell::Transparent
                }
            })
                .collect();

        let lost = false;

        //background = back_lines.join("\n");
        background = back_lines.join("");

        log(format!("BCG LENGTH {}", background.len()).as_str());

        log(s.as_str());
        log("-----------===========------------");
        log(background.as_str());

        Universe {
            lost,
            width,
            height,
            cells,
            snake,
            background,
            apple: 5,
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index1, line) in self.cells.as_slice().chunks(self.width as usize).enumerate() {
            for index2 in 0..line.len() {
                let symbol =
                    if line[index2] == Cell::Dead { '◻' } else if { line[index2] == Cell::Alive } { '◼' } else if { line[index2] == Cell::Apple } {log(format!("apple: {}", self.apple).as_str()); '◆'} else {
                        //log(format!("{} {:?}",index1, index2).as_str());
                        self.background.chars().nth(self.get_index(index2 as u32, index1 as u32)).unwrap_or('?')
                    };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}