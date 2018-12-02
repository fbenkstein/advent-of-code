extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, prelude::*};

fn count_reps<'a>(input: impl Iterator<Item = &'a Vec<u8>>, len: usize) -> usize {
    input
        .map(|x| {
            x.iter()
                .group_by(|c| *c)
                .into_iter()
                .any(|(_, list)| list.count() == len) as usize
        })
        .sum()
}

fn solve_part1(mut input: Vec<Vec<u8>>) {
    input.iter_mut().for_each(|x| x.sort());
    println!(
        "Checksum: {}",
        count_reps(input.iter(), 2) * count_reps(input.iter(), 3)
    );
}

fn solve_part2(input: &Vec<String>) {
    let mut seen = HashSet::new();
    for x in input {
        let substrs: Vec<_> = (0..x.len())
            .map(|i| {
                let mut sub = x.clone();
                sub.remove(i);
                sub
            })
            .collect();
        for sub in &substrs {
            if seen.contains(sub) {
                println!("Substring with only one char different: {}", sub);
                return;
            }
        }
        for sub in substrs {
            seen.insert(sub);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    solve_part1(lines.iter().map(|x| x.clone().into_bytes()).collect());
    solve_part2(&lines);
}
