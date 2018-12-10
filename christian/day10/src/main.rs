#[macro_use]
extern crate text_io;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, prelude::*};

#[derive(Debug, Default, Clone)]
struct Point {
    pos: (isize, isize),
    speed: (isize, isize),
}

fn parse(input: &Vec<String>) -> Vec<Point> {
    let convert = |line: &String| {
        let mut point: Point = Default::default();
        scan!(line.bytes().filter(|&c| c != b' ') => "position=<{},{}>velocity=<{},{}>",point.pos.0,point.pos.1,point.speed.0,point.speed.1);
        point
    };
    input.iter().map(convert).collect()
}

fn print_solution(mut input: Vec<Point>, iterations: isize) -> Option<()> {
    for x in input.iter_mut() {
        x.pos.0 += x.speed.0 * iterations;
        x.pos.1 += x.speed.1 * iterations;
    }

    let dim_x = input.iter().map(|p| p.pos.0).minmax().into_option()?;
    let dim_y = input.iter().map(|p| p.pos.1).minmax().into_option()?;
    let mut matrix = vec![' '; ((dim_x.1 - dim_x.0 + 1) * (dim_y.1 - dim_y.0 + 1)) as usize];
    for x in input {
        matrix[(x.pos.0 - dim_x.0 + (x.pos.1 - dim_y.0) * (dim_x.1 - dim_x.0 + 1)) as usize] = '*';
    }

    for (i, c) in matrix.iter().enumerate() {
        print!("{}", c);
        if (i + 1) as isize % (dim_x.1 - dim_x.0 + 1) == 0 {
            println!("");
        }
    }
    Some(())
}

fn solve(input: &Vec<Point>) {
    let mut freqs = HashMap::new();
    for p1 in input {
        for p2 in input {
            // check if the two points can be aligned on x axis
            let diff = (p1.pos.0 - p2.pos.0, p1.pos.1 - p2.pos.1);
            let diff_speed = (p2.speed.0 - p1.speed.0, p2.speed.1 - p1.speed.1);
            if diff_speed.0 != 0
                && diff_speed.0.signum() == diff.0.signum()
                && diff.0 % diff_speed.0 == 0
            {
                // check if they would end up next to each other on y axis
                let iterations = diff.0 / diff_speed.0;
                let distance = diff.1 - diff_speed.1 * iterations;
                if distance == 1 || distance == -1 {
                    *freqs.entry(iterations).or_insert(0) += 1;
                }
            }
        }
    }

    let best_correlation = freqs.iter().max_by_key(|(_, &count)| count).unwrap();
    println!("Num Iterations: {:?}", best_correlation);

    print_solution(input.clone(), *best_correlation.0);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    solve(&input);
}
