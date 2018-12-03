use std::collections::HashSet;
use std::env;

fn main() {
    // -- part one --
    let args: Vec<String> = env::args().collect();
    let change_seq: Vec<i32> = args
        .into_iter()
        .skip(1)
        .map(|s| {
            s.parse::<i32>()
                .expect("Invalid input! Only space seperated integers allowed")
        }).collect();
    let result_freq: i32 = change_seq.iter().sum();
    println!("The resulting frequency is: {}", result_freq);

    // -- part two --
    let mut known_freq = HashSet::new();
    let mut current_freq: i32 = 0;
    let mut found: bool = false;
    while !found {
        for change in &change_seq {
            if known_freq.contains(&current_freq) {
                found = true;
                break;
            }
            known_freq.insert(current_freq);
            current_freq += change;
        }
    }
    println!("The first frequency occuring twice is: {}", current_freq);
}
