#[macro_use]
extern crate text_io;
#[macro_use]
extern crate itertools;

use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<isize> {
    let convert = |line: &String| {
        let serial;
        scan!(line.bytes().filter(|&c| c != b' ') => "{}", serial);
        serial
    };
    input.iter().map(convert).collect()
}

fn solve2(input: isize, max_size: usize) {
    let cost =
        |x, y| (((y as isize * (x as isize + 10) + input) * (x as isize + 10)) % 1000) / 100 - 5;
    let mut prefix_sums = vec![0; 302 * 302];
    let p = |x, y| x + y * 302;
    for (y, x) in iproduct!(1..302, 2..302) {
        prefix_sums[p(x, y)] = cost(x - 1, y) + prefix_sums[p(x - 1, y)];
    }
    let best = iproduct!(1..299, 1..299, 3..=max_size)
        .filter(|(x, y, size)| x + size <= 301 && y + size <= 301)
        .map(|(x, y, size)| {
            (
                (y..y + size)
                    .map(|row| prefix_sums[p(x + size, row)] - prefix_sums[p(x, row)])
                    .sum::<isize>(),
                (x, y, size),
            )
        })
        .max()
        .unwrap();
    println!("Serial {}, max size {}, best: {:?}", input, max_size, best);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    for x in input {
        solve2(x, 3);
        solve2(x, 299);
    }
}
