mod parser;

use crate::parser::Board;

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");
    let board = Board::from_str(&contents).expect("could not parse board.");
    println!("{}", board);

    // 1. scan for targets on board
    // 2. find all open squares that are reachable by an enemy
    // 3.
}
