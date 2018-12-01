use int_hash::IntHashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let frequency_changes: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|l| {
            l.expect("could not read file buffer")
                .parse()
                .expect("could not parse int")
        })
        .collect();

    let mut current_frequency: i32 = 0;
    let mut frequencies: IntHashMap<i32, u32> = IntHashMap::default();

    loop {
        frequency_changes.iter().for_each(|frequency_change| {
            current_frequency += frequency_change;
            *frequencies.entry(current_frequency).or_insert(0) += 1;
            if *frequencies.entry(current_frequency).or_default() == 2 {
                panic!(format!("Device calibrated! {}", current_frequency));
            }
        });
        println!("Calibrated frequency: {}", current_frequency);
    }
}
