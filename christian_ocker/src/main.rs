mod day1;
mod day2;
mod day5;

#[macro_use]
extern crate itertools;

use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let day: u8 = args.next().unwrap().parse().unwrap();
    let input_file = args.next().unwrap();

    let mut input = String::new();
    File::open(input_file)?.read_to_string(&mut input)?;

    match day {
        1 => println!("{:?}", day1::solve(&input)),
        2 => println!("{:?}", day2::solve(&input)),
        5 => println!("{:?}", day5::solve(&input)),
        _ => eprintln!("invalid day: {}", day),
    }
    Ok(())
}
