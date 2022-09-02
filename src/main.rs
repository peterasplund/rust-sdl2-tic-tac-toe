#![allow(dead_code)]

extern crate sdl2;

mod engine;
mod interface;

use self::engine::{Engine};

fn main() {
    let board = Engine::new(engine::Player::X);
    interface::run(board);
}
