mod day1;

use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let input_file = args.next().unwrap();
    let mut input = String::new();
    File::open(input_file)?.read_to_string(&mut input)?;
    println!("{:?}", day1::solve(&input));
    Ok(())
}
