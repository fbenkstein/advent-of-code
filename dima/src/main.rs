mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
        _ => eprintln!("invalid day: {}", day),
    }
    Ok(())
}
