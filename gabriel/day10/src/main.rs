mod parser;

use crate::parser::LightPoint;

use std::fs::File;
use std::io::prelude::*;

fn find_neighbors(light_points: &Vec<LightPoint>) -> bool {
    for point in light_points.iter() {
        if !point.has_neighbor(&light_points) {
            return false;
        }
    }
    return true;
}

fn print(points: &Vec<LightPoint>) {
    let min_x = points.iter().map(|p| p.x).min().expect("no min?");
    let max_x = points.iter().map(|p| p.x).max().expect("no max?");
    let offset_x = if min_x < 0 { min_x.abs() + 1 } else { -min_x };
    let width = (max_x + offset_x) as usize;

    let min_y = points.iter().map(|p| p.y).min().expect("no min?");
    let max_y = points.iter().map(|p| p.y).max().expect("no max?");
    let offset_y = if min_y < 0 { min_y.abs() + 1 } else { -min_y };
    let height = (max_y + offset_y) as usize;

    let mut buffer = vec![vec!['-'; width + 1]; height + 1];
    for point in points.iter() {
        let x = (point.x + offset_x) as usize;
        let y = (point.y + offset_y) as usize;
        buffer[y][x] = '*';
    }
    for line in buffer.iter() {
        println!("{}", line.iter().cloned().collect::<String>());
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");
    let (_, mut light_points) =
        parser::parse_light_points(nom::types::CompleteStr(&contents)).expect("could not parse");

    let mut frame = 1;
    loop {
        light_points.iter_mut().for_each(|p| p.translate());

        if find_neighbors(&light_points) {
            println!("Frame #{} looks good!", frame);
            print(&light_points);
            return;
        }
        frame += 1;
    }
}
