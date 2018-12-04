#[macro_use]
extern crate text_io;
extern crate itertools;

use itertools::Itertools;
use std::io::{self, prelude::*};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Action {
    NewGuard(u32),
    Sleep,
    Awake,
}

#[derive(Debug, PartialEq, Default, PartialOrd, Eq, Ord)]
struct Time {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Record {
    time: Time,
    action: Action,
}

fn parse(input: &Vec<String>) -> Vec<Record> {
    let convert = |x: &String| {
        let mut time: Time = Default::default();
        let mut next: String;
        let mut iter = x.bytes();
        scan!(iter => "[{}-{}-{} {}:{}] {}",time.year, time.month, time.day,time.hour,time.minute, next);
        let action = match &next[..] {
            "Guard" => {
                let number: u32;
                scan!(iter=>"#{} begins shift", number);
                Action::NewGuard(number)
            }
            "falls" => Action::Sleep,
            "wakes" => Action::Awake,
            _ => panic!(),
        };
        Record { time, action }
    };
    input.iter().map(convert).collect()
}

fn generate_minute_map(mut input: Vec<Record>) -> Vec<(u32, u32)> {
    input.sort_unstable();

    let mut map = Vec::new();
    let (mut guard, mut last_minute) = (0, 0);
    for record in input {
        if let Action::Awake = record.action {
            (last_minute..record.time.minute).for_each(|x| map.push((guard, x)));
        }
        if let Action::NewGuard(number) = record.action {
            guard = number;
        }
        last_minute = record.time.minute;
    }
    map.sort_unstable();
    map
}

fn max_occ_by_key<Item, Key>(
    iter: impl Iterator<Item = Item>,
    key_gen: impl FnMut(&Item) -> Key,
) -> (usize, Key)
where
    Key: Eq + Ord,
{
    iter.group_by(key_gen)
        .into_iter()
        .map(|(key, items)| (items.count(), key))
        .max()
        .unwrap()
}

fn solve(map: &Vec<(u32, u32)>) {
    let most_sleepy = max_occ_by_key(map.iter(), |(guard, _)| *guard);

    println!("Most sleepy guard: {:?}", most_sleepy);

    let which = max_occ_by_key(
        map.iter().filter(|(guard, _)| *guard == most_sleepy.1),
        |(_, minute)| *minute,
    );
    println!("    Most sleepy minute: {:?}", which);
    println!("    Checksum part 1: {}", which.1 * most_sleepy.1);

    let which = max_occ_by_key(map.iter(), |x| *x);
    println!("Most sleepy guard+minute: {:?}", which);
    println!("    Checksum part 2: {}", (which.1).0 * (which.1).1);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);

    let map = generate_minute_map(input);
    solve(&map);
}
