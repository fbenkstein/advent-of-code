mod parser;

use crate::parser::Board;

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use env_logger;

fn main() {
    env_logger::init();

    let mut file = File::open("input3.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");
    let mut board = Board::from_str(&contents).expect("could not parse board.");

    for _ in 0..3 {
        board.next_turn();
        println!("{}", board);
    }
}
