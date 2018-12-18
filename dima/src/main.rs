mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use regex;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<Error>> {
    let mut args = std::env::args().skip(1);
    let input_file = args.next().ok_or("Usage: advent-of-code-2018 <day.txt>")?;

    let re = regex::Regex::new(r"^.*day(\d+).txt$")?;
    let day: u8 = re
        .captures(&input_file)
        .and_then(|c| c.get(1).map(|g| g.as_str()))
        .ok_or_else(|| format!("can't deduce day from: {}", input_file))?
        .parse()?;

    let mut input = String::new();
    File::open(input_file)?.read_to_string(&mut input)?;

    match day {
        1 => println!("{:?}", day1::solve(&input)),
        2 => println!("{:?}", day2::solve(&input)),
        3 => println!("{:?}", day3::solve(&input)),
        4 => println!("{:?}", day4::solve(&input)),
        5 => println!("{:?}", day5::solve(&input)),
        6 => println!("{:?}", day6::solve(&input)),
        7 => println!("{:?}", day7::solve(&input)),
        8 => println!("{:?}", day8::solve(&input)),
        9 => println!("{:?}", day9::solve(&input)),
        10 => println!("{:?}", day10::solve(&input)),
        11 => println!("{:?}", day11::solve(&input)),
        12 => println!("{:?}", day12::solve(&input)),
        13 => println!("{:?}", day13::solve(&input)),
        14 => println!("{:?}", day14::solve(&input)),
        15 => println!("{:?}", day15::solve(&input)),
        16 => println!("{:?}", day16::solve(&input)),
        17 => println!("{:?}", day17::solve(&input)),
        18 => println!("{:?}", day18::solve(&input)),
        _ => eprintln!("invalid day: {}", day),
    }
    Ok(())
}
