#[macro_use]
extern crate text_io;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<(usize, usize)> {
    let convert = |line: &String| {
        let (players, max_marble): (usize, usize);
        scan!(line.bytes() => "{} players; last marble is worth {} points",players,max_marble);
        (players, max_marble)
    };
    input.iter().map(convert).collect()
}

fn solve(players: usize, max_marble: usize) {
    let mut points = vec![0; players];
    let mut list = vec![(0, 0); max_marble + 1];
    let mut current = 0;
    for x in 1..=max_marble {
        if x % 23 == 0 {
            points[x % players] += x;
            // move 7 left and remove
            for _ in 0..7 {
                current = list[current].0;
            }
            points[x % players] += current;
            let current_entry = list[current];
            list[current_entry.0].1 = current_entry.1;
            list[current_entry.1].0 = current_entry.0;
            current = list[current].1;
        } else {
            // insert -> insert between next and one after
            current = list[current].1;
            let next = list[current].1;
            list[current].1 = x;
            list[x].0 = current;
            list[x].1 = next;
            list[next].0 = x;
            current = x;
        }
    }
    let best_score = points.iter().max().unwrap();
    println!("Best score: {}", best_score);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    for (players, max_marble) in input {
        solve(players, max_marble);
    }
}
