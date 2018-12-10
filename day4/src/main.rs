mod parser;

use crate::parser::Action;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");
    let (not_parsed, mut events) = parser::parse_claims(parser::CompleteStr(&contents)).unwrap();
    if not_parsed.len() > 0 {
        println!();
        panic!(format!(
            "Nothing should be not_parsed from the parser! Did not parse: {:?}",
            not_parsed
        ));
    }

    events.sort();

    let mut last_guard_id = 0;
    let mut total_asleep: HashMap<u32, u32> = HashMap::new();
    let mut asleep: HashMap<u32, HashMap<u8, u32>> = HashMap::new();
    for (event, next_event) in events.iter().zip(events.iter().skip(1)) {
        match (&event.action, &next_event.action) {
            (Action::BeginsShift(id), _) => {
                last_guard_id = *id;
                println!("{}", last_guard_id);
            }
            (Action::FallsAsleep, Action::WakesUp) => {
                for minute in event.date.minute..next_event.date.minute {
                    *total_asleep.entry(last_guard_id).or_insert(0) += 1;
                    *asleep
                        .entry(last_guard_id)
                        .or_insert(HashMap::new())
                        .entry(minute)
                        .or_insert(0) += 1;
                }
            }
            _ => continue,
        }
    }

    let guard_id_that_sleeps_most = total_asleep
        .iter()
        .max_by(|&(_, a), &(_, b)| a.cmp(&b))
        .unwrap()
        .0;
    println!(
        "Guard sleeping the most is #{:?}",
        guard_id_that_sleeps_most
    );

    let minute_most_slept = asleep
        .get(guard_id_that_sleeps_most)
        .unwrap()
        .iter()
        .max_by(|&(_, a), &(_, b)| a.cmp(&b))
        .unwrap()
        .0;

    println!(
        "Best minute to sneak-in: {:#?}",
        guard_id_that_sleeps_most * *minute_most_slept as u32
    );

    let guard_asleep_more_than_any_other_guard = asleep
        .iter()
        .max_by(|&(_, a), &(_, b)| {
            let minute_most_slept_a = a.iter().max_by(|&(_, a), &(_, b)| a.cmp(&b));
            let minute_most_slept_b = b.iter().max_by(|&(_, a), &(_, b)| a.cmp(&b));
            minute_most_slept_a.cmp(&minute_most_slept_b)
        })
        .unwrap();

    println!("{:#?}", guard_asleep_more_than_any_other_guard);
    println!(
        "{:?}",
        guard_asleep_more_than_any_other_guard.0
            * *guard_asleep_more_than_any_other_guard
                .1
                .iter()
                .max_by(|&(_, a), &(_, b)| a.cmp(&b))
                .unwrap()
                .0 as u32
    );
}
