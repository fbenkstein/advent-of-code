extern crate regex;
#[macro_use]
extern crate itertools;

use regex::Regex;
use std::cmp;
use std::io::{self, prelude::*};

#[derive(Debug)]
struct Rect {
    id: usize,
    x: usize,
    y: usize,
    x_dim: usize,
    y_dim: usize,
}

fn parse(input: &Vec<String>) -> Vec<Rect> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    input
        .iter()
        .map(|x| {
            let groups = re.captures(x).unwrap();
            Rect {
                id: groups[1].parse().unwrap(),
                x: groups[2].parse().unwrap(),
                y: groups[3].parse().unwrap(),
                x_dim: groups[4].parse().unwrap(),
                y_dim: groups[5].parse().unwrap(),
            }
        })
        .collect()
}

fn max_coord(input: &Vec<Rect>) -> (usize, usize) {
    input.iter().fold((0, 0), |(max_x, max_y), x| {
        (
            cmp::max(cmp::max(max_x, x.x), x.x + x.x_dim),
            cmp::max(cmp::max(max_y, x.y), x.y + x.y_dim),
        )
    })
}

fn solve(input: &Vec<Rect>) {
    let dim = max_coord(input);
    let mut data = Vec::with_capacity(dim.0 * dim.1);
    data.resize(dim.0 * dim.1, 0_usize);

    for rect in input {
        for (x, y) in iproduct!(rect.x..rect.x + rect.x_dim, rect.y..rect.y + rect.y_dim) {
            data[x + y * dim.0] += 1;
        }
    }

    println!(
        "Num contested squares: {}",
        data.iter().filter(|x| **x > 1).count()
    );

    for rect in input {
        if iproduct!(rect.x..rect.x + rect.x_dim, rect.y..rect.y + rect.y_dim)
            .all(|(x, y)| data[x + y * dim.0] == 1)
        {
            println!("Not contested: {:?}", rect);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);

    solve(&input);
}
