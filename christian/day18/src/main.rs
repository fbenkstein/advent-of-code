#[macro_use]
extern crate itertools;

use std::collections::{hash_map, HashMap};
use std::io::{self, prelude::*};

fn count(data: &Vec<Vec<u8>>, x: usize, y: usize, c: u8) -> usize {
    iproduct!(
        x.saturating_sub(1)..data[0].len().min(x + 2),
        y.saturating_sub(1)..data.len().min(y + 2)
    )
    .filter(move |&pos| pos != (x, y))
    .map(move |pos| data[pos.1][pos.0])
    .filter(|&other| other == c)
    .count()
}

fn count_all(data: &Vec<Vec<u8>>, c: u8) -> usize {
    data.iter()
        .flat_map(|line| line.iter())
        .filter(|&&x| x == c)
        .count()
}

fn print(iter: usize, data: &Vec<Vec<u8>>) {
    println!("\n\nIteration {}", iter);
    for line in data {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
}

fn simulate(input: &Vec<Vec<u8>>, max_iter: usize) -> String {
    let mut now = input.clone();
    let mut next = input.clone();
    let mut lookup = HashMap::new();
    let mut iter = 0;
    while iter < max_iter {
        match lookup.entry(now.clone()) {
            hash_map::Entry::Occupied(x) => {
                let length = iter - *x.get();
                let skip = (max_iter - iter) / length * length;
                iter += skip;
                if skip > 0 {
                    println!("Skip iterations: {}", skip);
                }
            }
            hash_map::Entry::Vacant(x) => {
                x.insert(iter);
            }
        }
        print(iter + 1, &now);
        for (y, line) in now.iter().enumerate() {
            for (x, &c) in line.iter().enumerate() {
                next[y][x] = match c {
                    b'.' if count(&now, x, y, b'|') >= 3 => b'|',
                    b'|' if count(&now, x, y, b'#') >= 3 => b'#',
                    b'#' if count(&now, x, y, b'#') == 0 || count(&now, x, y, b'|') == 0 => b'.',
                    _ => c,
                };
            }
        }
        std::mem::swap(&mut now, &mut next);
        iter += 1;
    }
    let wood = count_all(&now, b'|');
    let yards = count_all(&now, b'#');
    format!("{} wood * {} yards = {}", wood, yards, wood * yards)
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin
        .lock()
        .lines()
        .map(|x| x.unwrap().into_bytes())
        .collect();
    let first = simulate(&lines, 10);
    let second = simulate(&lines, 1000000000);

    println!("{}\n{}", first, second);
}
