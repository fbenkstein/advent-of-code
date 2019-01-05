use std::mem;
use text_io::{scan, try_scan};

#[derive(Debug, Default)]
struct Point(i64, i64, i64, i64);

impl Point {
    fn dist(&self, other: &Self) -> i64 {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
    }
}

pub fn solve(input: &str) -> usize {
    let points = input.lines().map(|s| {
        let mut p = Point::default();
        scan!(s.bytes() => "{},{},{},{}", p.0, p.1, p.2, p.3);
        p
    });

    let mut constellations: Vec<Vec<Point>> = Vec::new();
    for p in points {
        let in_range: Vec<_> = constellations
            .iter()
            .enumerate()
            .filter_map(|(idx, set)| {
                if set.iter().any(|q| q.dist(&p) <= 3) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();
        if let Some(&first_idx) = in_range.first() {
            constellations[first_idx].push(p);
            for &idx in in_range.iter().skip(1) {
                let mut set = Vec::new();
                mem::swap(&mut set, &mut constellations[idx]);
                constellations[first_idx].append(&mut set);
            }
        } else {
            constellations.push(vec![p]);
        }
    }

    constellations.iter().filter(|c| !c.is_empty()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        const INPUT: &str = r#"0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0"#;
        assert_eq!(solve(INPUT), 2);
    }

    #[test]
    fn test_solve2() {
        const INPUT: &str = r#"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"#;
        assert_eq!(solve(INPUT), 4);
    }

    #[test]
    fn test_solve3() {
        const INPUT: &str = r#"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"#;
        assert_eq!(solve(INPUT), 3);
    }

    #[test]
    fn test_solve4() {
        const INPUT: &str = r#"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"#;
        assert_eq!(solve(INPUT), 8);
    }
}
