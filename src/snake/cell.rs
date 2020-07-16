use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::core::fmt::{Formatter, Error};

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
    Transparent = 2,
    Apple = 3,
}

impl Cell {
    pub fn switch(&mut self){
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
            Cell::Transparent => Cell::Transparent,
            Cell::Apple => Cell::Dead,
        };
    }

    pub fn new() -> Cell {
        Cell::Dead
    }
}