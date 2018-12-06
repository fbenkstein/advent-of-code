#[macro_use]
extern crate text_io;
use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<(isize, isize)> {
    let convert = |line: &String| {
        let (x, y);
        scan!(line.bytes() => "{},{}",x,y);
        (x, y)
    };
    input.iter().map(convert).collect()
}

fn solve1(input: &Vec<(isize, isize)>) {
    let dim_x = input.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let dim_y = input.iter().map(|(_, y)| *y).max().unwrap() + 1;
    let invalid = input.len();
    let mut seen = vec![(std::usize::MAX, invalid); (dim_y * dim_x) as usize];
    let mut queue = Vec::new();
    for (i, (x, y)) in input.iter().enumerate() {
        queue.push((*x, *y));
        seen[(x + y * dim_x) as usize] = (0, i);
    }

    let mut infinite = vec![false; input.len() + 1];
    let mut count = vec![0; input.len() + 1];
    infinite[invalid] = true;
    let mut pos = 0;
    while pos < queue.len() {
        let (x, y) = queue[pos];
        let (dist, parent) = seen[(x + y * dim_x) as usize];
        count[parent] += 1;
        for (new_x, new_y) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter() {
            if *new_x < 0 || *new_y < 0 || *new_x >= dim_x || *new_y >= dim_y {
                infinite[parent] = true;
                continue;
            }
            let (target_dist, target_parent) = &mut seen[(new_x + new_y * dim_x) as usize];
            if *target_dist == dist + 1 && *target_parent != parent {
                *target_parent = invalid;
            } else if *target_dist > dist + 1 {
                queue.push((*new_x, *new_y));
                *target_dist = dist + 1;
                *target_parent = parent;
            }
        }
        pos += 1;
    }

    // find biggest non-infinite area
    let biggest = count.iter().max().unwrap();
    println!("Biggest area {:?}", biggest);
}

fn solve2(input: &Vec<(isize, isize)>) {
    let dim_x = input.iter().map(|(x, _)| *x).max().unwrap() as usize + 1;
    let dim_y = input.iter().map(|(_, y)| *y).max().unwrap() as usize + 1;
    // build prefix sums for quick lookup when moving on the grid
    let mut prefix_sum_x = vec![0; dim_x + 1];
    let mut prefix_sum_y = vec![0; dim_y + 1];
    for (x, y) in input {
        prefix_sum_y[*y as usize] += 1;
        prefix_sum_x[*x as usize] += 1;
    }
    for x in 1..=dim_x {
        prefix_sum_x[x] += prefix_sum_x[x - 1];
    }
    for y in 1..=dim_y {
        prefix_sum_y[y] += prefix_sum_y[y - 1];
    }

    // go through the grid and adjust cost cheaply
    let mut num_in_range = 0;
    let mut cost: isize = input.iter().map(|(x, y)| x + y).sum();
    for x in 0..dim_x {
        let mut local_cost = cost;
        for y in 0..dim_y {
            if local_cost < 10000 {
                num_in_range += 1;
            }
            local_cost += 2 * prefix_sum_y[y] - input.len() as isize;
        }
        cost += 2 * prefix_sum_x[x] - input.len() as isize;
    }

    println!("Num in range: {}", num_in_range);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    solve1(&input);
    solve2(&input);
}
