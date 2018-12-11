use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("file not found");

    let mut exactly_two = 0;
    let mut exactly_three = 0;

    let box_ids: Vec<String> = BufReader::new(file)
        .lines()
        .map(|l| {
            let mut line_occurences = HashMap::new();
            let line = l.expect("could not read file buffer");
            line.chars()
                .for_each(|c| *line_occurences.entry(c).or_insert(0) += 1);
            if line_occurences.values().find(|&o| *o == 2).is_some() {
                exactly_two += 1;
            }
            if line_occurences.values().find(|&o| *o == 3).is_some() {
                exactly_three += 1;
            }
            line
        })
        .collect();

    println!(
        "Checksum of the list of box IDs: {}",
        exactly_two * exactly_three
    );

    let fabric_box_id = box_ids
        .iter()
        .filter_map(|box_id| {
            box_ids
                .iter()
                .filter_map(|other_box_id| {
                    let positions: Vec<usize> = box_id
                        .chars()
                        .enumerate()
                        .zip(other_box_id.chars())
                        .filter_map(|((pos, a), b)| if a != b { Some(pos) } else { None })
                        .collect();
                    if positions.len() == 1 {
                        let mut fabric_box_id = other_box_id.clone();
                        fabric_box_id.remove(positions[0]);
                        Some(fabric_box_id)
                    } else {
                        None
                    }
                })
                .filter(|similar_box_ids| similar_box_ids.len() > 0)
                .next()
        })
        .next()
        .expect("Could not find the fabric box ID, bummer!");
    println!("Fabric box ID: {}", fabric_box_id);
}
