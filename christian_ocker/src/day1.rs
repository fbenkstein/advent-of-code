use std::collections::HashSet;
use std::iter;

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .sum::<i32>()
}

pub fn solve(input: &str) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new();
    iter::repeat(input.lines())
        .flatten()
        .map(|x| x.parse::<i32>().unwrap())
        .scan(0, |freq, freq_change| {
            *freq += freq_change;
            Some(*freq)
        }).find(|freq| !seen.insert(*freq))
        .unwrap()
}
