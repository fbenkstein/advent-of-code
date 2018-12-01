use itertools::{FoldWhile, Itertools};
use std::collections::BTreeSet;
use std::num::ParseIntError;
use std::str::FromStr;

fn parse(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.lines().map(i64::from_str).collect()
}

fn solve1(seq: &[i64]) -> i64 {
    seq.iter().sum()
}

fn solve2(seq: &[i64]) -> i64 {
    let mut seen = BTreeSet::new();
    seen.insert(0);

    seq.iter()
        .cycle()
        .fold_while(0, |sum, x| {
            let sum = sum + x;
            if !seen.insert(sum) {
                FoldWhile::Done(sum)
            } else {
                FoldWhile::Continue(sum)
            }
        })
        .into_inner()
}

pub fn solve(input: &str) -> Result<(i64, i64), ParseIntError> {
    parse(input).map(|seq| (solve1(&seq), solve2(&seq)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&vec![1, -1]), 0);
        assert_eq!(solve2(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(solve2(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(solve2(&vec![7, 7, -2, -7, -4]), 14);
    }
}
