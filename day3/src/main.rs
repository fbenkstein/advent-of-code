mod parser;

use std::fs::File;
use std::io::prelude::*;

fn part_two(canvas: &Vec<u8>, canvas_width: u64) -> u64 {
    // Because I suck at Rust, I can't iter twice but for now... \o/
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");
    let claims = parser::parse_claims(parser::CompleteStr(&contents))
        .unwrap()
        .1;

    let claims: Vec<&parser::Claim> = claims
        .iter()
        .filter(|&claim| {
            println!("{:?}", claim);
            for x in claim.top_left.x..claim.bottom_right.x {
                for y in claim.top_left.y..claim.bottom_right.y {
                    if canvas[(x + y * canvas_width) as usize] > 1 {
                        return false;
                    }
                }
            }
            return true;
        })
        .collect();
    claims[0].id
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");
    let claims = parser::parse_claims(parser::CompleteStr(&contents))
        .unwrap()
        .1;

    let canvas_size = claims.iter().fold((0, 0), |(x, y), claim| {
        (x.max(claim.bottom_right.x), y.max(claim.bottom_right.y))
    });

    let mut canvas: Vec<u8> = vec![0; (canvas_size.0 * canvas_size.1) as usize];

    claims.iter().for_each(|claim| {
        for x in claim.top_left.x..claim.bottom_right.x {
            for y in claim.top_left.y..claim.bottom_right.y {
                // basically, we stack the cells vertically
                canvas[(x + y * canvas_size.0) as usize] += 1;
            }
        }
    });

    let intersection_inches = canvas.iter().filter(|&inch| *inch >= 2).count();
    println!("{}", intersection_inches);

    let non_overlapping_claims = part_two(&canvas, canvas_size.0);
    println!("{:?}", non_overlapping_claims);
}
