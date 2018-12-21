use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");
    println!("Hello, world!");

    let mut positions = vec![];
    let (mut x, mut y) = (5000, 5000); // some high value
    let (mut prev_x, mut prev_y) = (x, y);

    let mut m: HashMap<(isize, isize), HashSet<(isize, isize)>> = HashMap::new();
    let mut distances: HashMap<(isize, isize), isize> = HashMap::new();

    for c in contents.chars() {
        match c {
            '(' => {
                positions.push((x, y));
            }
            ')' => {
                let pos = positions
                    .pop()
                    .expect("cannot pop position, wrong end-parenthesis?");
                x = pos.0;
                y = pos.1;
            }
            '|' => {
                let pos = positions.last().expect("no last position");
                x = pos.0;
                y = pos.1;
            }
            'N' | 'E' | 'S' | 'W' => {
                let (dx, dy) = match c {
                    'N' => (0, -1),
                    'E' => (1, 0),
                    'S' => (0, 1),
                    'W' => (-1, 0),
                    _ => unreachable!(),
                };
                x += dx;
                y += dy;
                m.entry((x, y)).or_default().insert((prev_x, prev_y));
                if *distances.entry((x, y)).or_default() != 0 {
                    let distance = *distances.entry((x, y)).or_default();
                    let previous_distance = *distances.entry((prev_x, prev_y)).or_default() + 1;
                    distances.insert((x, y), std::cmp::min(distance, previous_distance));
                } else {
                    let previous_distance = *distances.entry((prev_x, prev_y)).or_default() + 1;
                    distances.insert((x, y), previous_distance);
                };
            }
            _ => continue,
        }
        prev_x = x;
        prev_y = y;
    }

    println!(
        "Largest number of doors you would be required to pass through to reach a room {:?}",
        distances.values().max()
    );
    println!("{} rooms have a shortest path from your current location that pass through at least 1000 doors.", distances.values().filter(|&d| *d >= 1000).count());
}
