extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, prelude::*};

fn count_reps<'a>(input: impl Iterator<Item = &'a Vec<u8>>, len: usize) -> usize {
    let has_len = |x: &&Vec<u8>| {
        x.iter()
            .group_by(|c| *c)
            .into_iter()
            .any(|(_, list)| list.count() == len)
    };
    input.filter(has_len).count()
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
    for line in input {
        let create_substr = |i: usize| {
            let mut sub = line.clone();
            sub.remove(i);
            (i, sub)
        };
        let substrs: Vec<_> = (0..line.len()).map(create_substr).collect();
        for x in &substrs {
            if seen.contains(x) {
                println!("Substring with only one char different: {} @ {}", x.1, x.0);
                return;
            }
        }
        for x in substrs {
            seen.insert(x);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    solve_part1(lines.iter().map(|x| x.clone().into_bytes()).collect());
    solve_part2(&lines);
}
