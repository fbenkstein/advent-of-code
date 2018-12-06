mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
        3 => println!("{:?}", day3::solve(&input)),
        4 => println!("{:?}", day4::solve(&input)),
        5 => println!("{:?}", day5::solve(&input)),
        6 => println!("{:?}", day6::solve(&input)),
        _ => eprintln!("invalid day: {}", day),
    }
    Ok(())
}
