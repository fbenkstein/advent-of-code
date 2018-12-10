#[macro_use]
extern crate text_io;
use cgmath::{vec2, Vector2};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, prelude::*};

#[derive(Debug, Clone)]
struct Star {
    pos: Vector2<isize>,
    speed: Vector2<isize>,
}

fn parse(input: &Vec<String>) -> Vec<Star> {
    let convert = |line: &String| {
        let (x, y, speed_x, speed_y): (isize, isize, isize, isize);
        scan!(line.bytes().filter(|&c| c != b' ') => "position=<{},{}>velocity=<{},{}>", x, y, speed_x, speed_y);
        Star {
            pos: vec2(x, y),
            speed: vec2(speed_x, speed_y),
        }
    };
    input.iter().map(convert).collect()
}

fn print_solution(mut input: Vec<Star>, iterations: isize) -> Option<()> {
    for star in input.iter_mut() {
        star.pos += star.speed * iterations;
    }

    let dim_x = input.iter().map(|p| p.pos.x).minmax().into_option()?;
    let dim_y = input.iter().map(|p| p.pos.y).minmax().into_option()?;
    let mut matrix = vec![' '; ((dim_x.1 - dim_x.0 + 1) * (dim_y.1 - dim_y.0 + 1)) as usize];
    for star in input {
        let pos = star.pos - Vector2::new(dim_x.0, dim_y.0);
        matrix[(pos.x + pos.y * (dim_x.1 - dim_x.0 + 1)) as usize] = '*';
    }

    for (i, c) in matrix.iter().enumerate() {
        print!("{}", c);
        if (i + 1) as isize % (dim_x.1 - dim_x.0 + 1) == 0 {
            println!("");
        }
    }
    Some(())
}

fn solve(input: &Vec<Star>) {
    let mut freqs = HashMap::new();
    for p1 in input {
        for p2 in input {
            // check if the two points can be aligned on x axis
            let diff = p1.pos - p2.pos;
            let diff_speed = p2.speed - p1.speed;
            if diff_speed.x != 0
                && diff_speed.x.signum() == diff.x.signum()
                && diff.x % diff_speed.x == 0
            {
                // check if they would end up next to each other on y axis
                let iterations = diff.x / diff_speed.x;
                let distance = diff.y - diff_speed.y * iterations;
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
