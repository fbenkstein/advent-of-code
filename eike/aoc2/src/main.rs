use std::collections::HashMap;
use std::env;

// the best function ever
fn differ_by_one(lhs: &str, rhs: &str) -> String {
    let differences = lhs
        .chars()
        .zip(rhs.chars())
        .map(|(left, right)| match left == right {
            true => Some(left),
            false => None,
        }).collect::<Vec<Option<_>>>();
    if differences.iter().fold(0, |acc, letter| {
        acc + match letter {
            Some(_abc) => 0,
            None => 1,
        }
    }) != 1
    {
        return "".to_string();
    }

    differences
        .iter()
        .filter_map(|&letter| letter)
        .collect::<String>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let box_ids = &args[1..];

    // --- part one ---
    let mut num_twos = 0;
    let mut num_threes = 0;
    for box_id in box_ids.iter() {
        let mut letter_counts = HashMap::new();
        for letter in box_id.chars() {
            let count = letter_counts.entry(letter).or_insert(0);
            *count += 1;
        }
        for (_letter, count) in letter_counts.iter() {
            if *count == 2 {
                num_twos += 1;
                break;
            }
        }
        for (_letter, count) in letter_counts.iter() {
            if *count == 3 {
                num_threes += 1;
                break;
            }
        }
    }

    println!("Checksum of all box IDs is: {}", num_twos * num_threes);

    // --- part two ---
    for left_box_id in box_ids {
        for right_box_id in box_ids {
            let result = differ_by_one(left_box_id, right_box_id);
            if !result.is_empty() {
                println!(
                    "Matching characters of single difference string is: {}",
                    result
                );
                return;
            }
        }
    }
}
