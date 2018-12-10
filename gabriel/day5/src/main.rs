use std::fs::File;
use std::io::prelude::*;

fn compactor(input: impl Iterator<Item=char>) -> String {
  let mut stack: Vec<char> = vec![];
  for c in input {
    if stack.last().map(|o| c.to_ascii_lowercase() == o.to_ascii_lowercase() && c.is_lowercase() != o.is_lowercase()).unwrap_or(false) {
      stack.pop();
    } else {
      stack.push(c);
    }
  }

  stack.into_iter().collect()
}

fn polymer_improve(input: &str) -> Option<usize> {
  let mut units: Vec<char> = input.to_ascii_lowercase().chars().collect();
  units.sort();
  units.dedup();
  println!("{:?}", units);

  units.into_iter().map(|unit| {
    compactor(input.chars().filter(|c| c.to_ascii_lowercase() != unit)).len()
  }).min()
}

fn main() {
  let mut file = File::open("input.txt").expect("file not found");
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("could not read file");
  println!("Number of polymer units: {}", compactor(contents.chars()).len());

  println!("Length of the shortest polymer: {:?}", polymer_improve(&contents));
}

#[test]
fn test_compactor() {
  assert_eq!(compactor("dabAcCaCBAcCcaDA".chars()), "dabCBAcaDA");
  assert_eq!(compactor("CC".chars()), "CC");
  assert_eq!(compactor("YyLlXxYKkbNnQqBFfxXbyYWwBhHyYTCBbCjIiqwtTWQJczeEauUAZDdFfm".chars()), "YTCm");
  assert_eq!(compactor("dabAcCaCBAcCcaDA".chars()).len(), 10);
}