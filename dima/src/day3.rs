use lazy_static::lazy_static;
use regex::{Match, Regex};

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }
        let error_msg = || format!("cannot parse claim: {}", s);
        let parse_match = |capture: Option<Match>| {
            capture
                .ok_or_else(error_msg)
                .and_then(|m| str::parse(m.as_str()).map_err(|_| error_msg()))
        };

        RE.captures(s).ok_or_else(error_msg).and_then(|capture| {
            Ok(Claim {
                id: parse_match(capture.get(1))?,
                left: parse_match(capture.get(2))?,
                top: parse_match(capture.get(3))?,
                width: parse_match(capture.get(4))?,
                height: parse_match(capture.get(5))?,
            })
        })
    }
}

#[derive(Debug, Default)]
struct Fabric(HashMap<(u32, u32), (HashSet<u32>, u32)>);

impl Fabric {
    fn add_claim(&mut self, claim: &Claim) {
        for i in claim.left..claim.left + claim.width {
            for j in claim.top..claim.top + claim.height {
                self.0
                    .entry((i, j))
                    .and_modify(|(ids, count)| {
                        ids.insert(claim.id);
                        *count += 1
                    })
                    .or_insert_with(|| {
                        let mut ids = HashSet::new();
                        ids.insert(claim.id);
                        (ids, 1)
                    });
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = &(HashSet<u32>, u32)> {
        self.0.values()
    }
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Result<Claim, String>> + 'a {
    input.lines().map(Claim::from_str)
}

pub fn solve(input: &str) -> Result<(u32, HashSet<u32>), String> {
    let claims: Result<Vec<Claim>, _> = parse(input).collect();
    let claims = claims?;

    let mut fabric = Fabric::default();
    for claim in &claims {
        fabric.add_claim(claim);
    }

    let all_ids: HashSet<u32> = claims.iter().map(|claim| claim.id).collect();
    let res = fabric.iter().fold(
        (0, all_ids),
        |(mut overlapping_area, mut non_overlapping_ids), (ids, count)| {
            if *count >= 2 {
                overlapping_area += 1;
                for id in ids {
                    non_overlapping_ids.remove(id);
                }
            }
            (overlapping_area, non_overlapping_ids)
        },
    );
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let parsed = Claim::from_str("#1397 @ 888,761: 25x24");
        assert_eq!(
            parsed,
            Ok(Claim {
                id: 1397,
                left: 888,
                top: 761,
                width: 25,
                height: 24,
            })
        );
    }

    #[test]
    fn test_solve() {
        let ids: HashSet<_> = [3u32].iter().cloned().collect();
        assert_eq!(
            solve(
                r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#
            ),
            Ok((4, ids))
        );
    }
}
