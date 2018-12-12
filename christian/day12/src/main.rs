#[macro_use]
extern crate text_io;
use std::io::{self, prelude::*};
use std::ops::Range;

struct Input {
    initial: Vec<bool>,
    rules: Vec<(Vec<bool>, bool)>,
}

fn parse(input: &Vec<String>) -> Input {
    let to_bool = |c| c == '#';
    let state: String;
    scan!((&input[0]).bytes() => "initial state: {}", state);
    let convert = |line: &String| {
        let (from, to): (String, char);
        scan!(line.bytes() => "{} => {}", from, to);
        (from.chars().map(to_bool).collect(), to_bool(to))
    };
    Input {
        initial: state.chars().map(to_bool).collect(),
        rules: input[2..].iter().map(convert).collect(),
    }
}

fn active_range(state: &Vec<bool>) -> Range<usize> {
    Range {
        start: state.iter().position(|x| *x).unwrap(),
        end: state.len() - state.iter().rev().position(|x| *x).unwrap() + 1,
    }
}

fn solve(Input { initial, rules }: &Input, max_iter: usize, num_iter: usize) {
    let mut state = vec![false; max_iter * 2];
    state.extend(initial.iter());
    state.extend(vec![false; max_iter * 2]);

    let mut offset: isize = 0;
    for iter in 0..max_iter {
        let mut next_state = vec![false; state.len()];
        for pos in 0..state.len() - 5 {
            let rule = rules.iter().find(|rule| rule.0[..] == state[pos..pos + 5]);
            next_state[pos + 2] = rule.map(|rule| rule.1).unwrap_or(false);
        }
        let (prev, next) = (active_range(&state), active_range(&next_state));
        if state[prev.clone()] == next_state[next.clone()] {
            println!("Stabilized {}: {} -> {}", iter, prev.start, next.start);
            offset = (num_iter - iter) as isize * (next.start as isize - prev.start as isize);
            break;
        }
        state = next_state;
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
