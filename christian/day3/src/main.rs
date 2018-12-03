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

fn solve(input: &Vec<Rect>) {
    let dim = input.iter().fold((0, 0), |(x, y), r| {
        (cmp::max(x, r.x + r.x_dim), cmp::max(y, r.y + r.y_dim))
    });
    let mut data = Vec::with_capacity(dim.0 * dim.1);
    data.resize(dim.0 * dim.1, 0_usize);

    let cells =
        |r: &Rect| iproduct!(r.x..r.x + r.x_dim, r.y..r.y + r.y_dim).map(|(x, y)| x + y * dim.1);

    input.iter().map(cells).flatten().for_each(|x| data[x] += 1);

    println!("#Squares > 1: {}", data.iter().filter(|&&x| x > 1).count());

    for r in input.iter().filter(|r| cells(r).all(|x| data[x] == 1)) {
        println!("Not contested: {:?}", r);
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);

    solve(&input);
}
