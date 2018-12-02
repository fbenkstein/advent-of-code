use itertools::iproduct;
use std::collections::BTreeMap;

fn solve1(input: &str) -> usize {
    let (has_2_count, has_3_count) = input
        .lines()
        .map(|s| {
            let mut occurences: BTreeMap<char, usize> = BTreeMap::new();
            for c in s.chars() {
                occurences
                    .entry(c)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            occurences
                .values()
                .fold((0, 0), |(has_2, has_3), count| match count {
                    2 => (1, has_3),
                    3 => (has_2, 1),
                    _ => (has_2, has_3),
                })
        })
        .fold((0, 0), |(twos, threes), (a, b)| (twos + a, threes + b));

    has_2_count * has_3_count
}

fn distance(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .map(|(x, y)| if x == y { 0 } else { 1 })
        .sum::<usize>()
        + a.len()
            .saturating_sub(b.len())
            .max(a.len().saturating_sub(b.len()))
}

fn solve2(input: &str) -> Option<String> {
    iproduct!(input.lines(), input.lines())
        .find(|(a, b)| distance(a, b) == 1)
        .map(|(a, b)| {
            a.chars()
                .zip(b.chars())
                .filter_map(|(x, y)| if x == y { Some(x) } else { None })
                .collect()
        })
}

pub fn solve(input: &str) -> (usize, Option<String>) {
    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = r#"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"#;

        assert_eq!(solve1(input), 12);
    }

    #[test]
    fn test_solve2() {
        let input = r#"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"#;
        assert_eq!(solve2(input), Some("fgij".into()));
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance("abcde", "axcye"), 2);
        assert_eq!(distance("fghij", "fguij"), 1);
    }
}
