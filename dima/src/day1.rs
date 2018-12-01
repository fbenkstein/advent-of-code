use itertools::{FoldWhile, Itertools};
use std::collections::BTreeSet;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve(input: &str) -> Result<i64, ParseIntError> {
    input
        .lines()
        .map(i64::from_str)
        .fold_results(0, |acc, x| acc + x)
}

pub fn solve2(input: &str) -> Result<i64, ParseIntError> {
    let seq: Result<Vec<_>, _> = input.lines().map(i64::from_str).collect();
    let seq = seq?;

    let mut seen = BTreeSet::new();
    seen.insert(0);

    let res = RingIterator::from(&seq)
        .fold_while(0, |mut sum, x| {
            sum += x;
            if !seen.insert(sum) {
                FoldWhile::Done(sum)
            } else {
                FoldWhile::Continue(sum)
            }
        })
        .into_inner();
    Ok(res)
}

struct RingIterator<'a, T> {
    vec: &'a Vec<T>,
    next: usize,
}

impl<'a, T> From<&'a Vec<T>> for RingIterator<'a, T> {
    fn from(vec: &'a Vec<T>) -> Self {
        Self { vec, next: 0 }
    }
}

impl<'a, T> Iterator for RingIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if !self.vec.is_empty() {
            let item = &self.vec[self.next];
            self.next += 1;
            self.next %= self.vec.len();
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve2() {
        assert_eq!(solve2("+1\n-1"), Ok(0));
        assert_eq!(solve2("+3\n+3\n+4\n-2\n-4"), Ok(10));
        assert_eq!(solve2("-6\n+3\n+8\n+5\n-6"), Ok(5));
        assert_eq!(solve2("+7\n+7\n-2\n-7\n-4"), Ok(14));
    }
}
