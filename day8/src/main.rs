mod model;
use crate::model::License;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let license = License::decode(&contents);
    println!("License metadata sum: {}", license.part_one());
    println!("Value of license root node: {}", license.part_two());
}

#[test]
fn test_decode_license_file() {
    let license = License::decode("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(license.part_one(), 138);
    assert_eq!(license.part_two(), 66);
}
