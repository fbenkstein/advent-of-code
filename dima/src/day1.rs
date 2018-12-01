use itertools::Itertools;
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

    let mut i = 0;
    let mut sum = 0;
    let mut seen = BTreeSet::new();

    loop {
        sum += seq[i];
        if !seen.insert(sum) {
            return Ok(sum);
        }

        i = (i + 1) % seq.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve2() {
        assert_eq!(solve2("+1\n-1"), Ok(1));
        assert_eq!(solve2("+3\n+3\n+4\n-2\n-4"), Ok(10));
        assert_eq!(solve2("-6\n+3\n+8\n+5\n-6"), Ok(5));
        assert_eq!(solve2("+7\n+7\n-2\n-7\n-4"), Ok(14));
    }
}
