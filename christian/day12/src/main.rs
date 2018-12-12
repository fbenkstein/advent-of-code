#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::io::{self, prelude::*};

struct Input {
    initial: Vec<bool>,
    rules: Vec<(Vec<bool>, bool)>,
}

fn parse(input: &Vec<String>) -> Input {
    let to_bool = |c| c == '#';
    let state: String;
    let first_line = &input[0];
    scan!(first_line.bytes() => "initial state: {}", state);
    let convert = |line: &String| {
        let from: String;
        let to: char;
        scan!(line.bytes() => "{} => {}", from, to);
        (from.chars().map(to_bool).collect(), to_bool(to))
    };
    Input {
        initial: state.chars().map(to_bool).collect(),
        rules: input[2..].iter().map(convert).collect(),
    }
}

fn solve(input: &Input, max_iter: usize, num_iter: usize) {
    let mut state = vec![false; max_iter * 4];
    state.extend(input.initial.iter());
    state.extend(vec![false; max_iter * 2]);

    let mut lookup: HashMap<Vec<bool>, usize> = HashMap::new();
    let mut offset: isize = 0;
    for iter in 0..max_iter {
        let mut next_state = vec![false; state.len()];
        for rule in input.rules.iter() {
            for pos in 0..state.len() - 5 {
                if rule.0[..] == state[pos..pos + 5] {
                    next_state[pos + 2] = rule.1;
                }
            }
        }
        state = next_state;
        let first = state.iter().position(|x| *x).unwrap();
        let last = state.len() - state.iter().rev().position(|x| *x).unwrap();
        if let Some(prev_first) = lookup.insert(state[first..=last].into(), first) {
            println!("Stabilized {}, offset {} -> {}", iter, prev_first, first);
            offset = (num_iter - 1 - iter) as isize * (first as isize - prev_first as isize);
            break;
        }
    }

    let eval = |(pos, x): (_, &_)| *x as isize * (pos as isize - (2 * max_iter as isize) + offset);
    let sum: isize = state.iter().enumerate().map(eval).sum();
    println!("Result of {} iterations: {}", num_iter, sum);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    solve(&input, 20, 20);
    solve(&input, 1000, 50000000000);
}
