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
    for i in 0..input[0].len() {
        let mut seen = HashSet::new();
        for line in input {
            let mut sub = line.clone();
            sub.remove(i);
            if !seen.insert(sub.clone()) {
                println!("Substring with only one char different: {}", sub);
                return;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    solve_part1(lines.iter().map(|x| x.clone().into_bytes()).collect());
    solve_part2(&lines);
}
