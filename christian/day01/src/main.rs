use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::iter::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let input: Vec<i32> = lines.map(|x| x.unwrap().parse().unwrap()).collect();
    let sum: i32 = input.iter().sum();
    println!("Sum: {}", input.iter().sum());
    let mut seen = HashSet::new();
    let mut sum = 0;
    for x in repeat(input.iter()).flatten() {
        if !seen.insert(sum) {
            println!("First repeated: {}", sum);
            break;
        }
        sum += x;
    }
}
