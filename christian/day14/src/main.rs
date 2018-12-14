#[macro_use]
extern crate text_io;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<usize> {
    let convert = |line: &String| {
        let x: usize;
        scan!(line.bytes() => "{}", x);
        x
    };
    input.iter().map(convert).collect()
}

fn solve(max: usize) {
    let mut state = vec![3, 7];
    let mut pos = (0, 1);
    while state.len() < max + 10 {
        let sum = state[pos.0] + state[pos.1];
        if sum >= 10 {
            state.push(sum / 10);
        }
        state.push(sum % 10);
        pos = (
            (state[pos.0] + 1 + pos.0) % state.len(),
            (state[pos.1] + 1 + pos.1) % state.len(),
        );
    }
    for x in max..max + 10 {
        print!("{}", state[x]);
    }
    println!("");
}

fn create_pattern(mut x: usize) -> Vec<u8> {
    let mut result = Vec::new();
    while x > 0 {
        result.push((x % 10) as u8);
        x /= 10;
    }
    result.reverse();
    result
}

fn solve2(max: usize) {
    let pattern = create_pattern(max);
    print!("Searching for ");
    for x in &pattern {
        print!("{}", x);
    }
    println!("");

    let mut state = vec![3_u8, 7_u8];
    let mut pos = (0, 1);
    let found = |state: &Vec<u8>, pattern: &Vec<u8>| {
        state.len() >= pattern.len() && state[state.len() - pattern.len()..] == pattern[..]
    };
    while !found(&state, &pattern) {
        let sum = state[pos.0] + state[pos.1];
        if sum >= 10 {
            state.push(sum / 10);
            if found(&state, &pattern) {
                break;
            }
        }
        state.push(sum % 10);
        pos = (
            (state[pos.0] as usize + 1 + pos.0) % state.len(),
            (state[pos.1] as usize + 1 + pos.1) % state.len(),
        );
    }
    println!("Found pattern after: {}", state.len() - pattern.len());
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    for x in input {
        solve(x);
        solve2(x);
    }
}
