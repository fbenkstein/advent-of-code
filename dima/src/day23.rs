use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::i64;
use text_io::{scan, try_scan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(i64, i64, i64);

impl Coord {
    fn dist(&self, other: &Self) -> i64 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs())
    }

    fn norm(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }

    fn min_coords(&self, other: &Self) -> Self {
        Coord(
            self.0.min(other.0),
            self.1.min(other.1),
            self.2.min(other.2),
        )
    }

    fn max_coords(&self, other: &Self) -> Self {
        Coord(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
        )
    }
}

fn parse(input: &str) -> Vec<(Coord, i64)> {
    input
        .lines()
        .map(|l| {
            let x: i64;
            let y: i64;
            let z: i64;
            let r: i64;
            scan!(l.bytes() => "pos=<{},{},{}>, r={}", x, y, z, r);
            (Coord(x, y, z), r)
        })
        .collect()
}

fn num_in_range_of_strongest(nanobots: &[(Coord, i64)]) -> usize {
    let (strongest_pos, strongest_r) = nanobots.iter().max_by_key(|(_, r)| r).unwrap();
    nanobots
        .iter()
        .filter(|(pos, _)| strongest_pos.dist(pos) <= *strongest_r)
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Quad {
    pos: Coord,      // corner with smallest coords
    width: i64,      // exclusive width
    num_bots: usize, // number of bots intersecting
}

impl Quad {
    fn new(pos: Coord, width: i64, nanobots: &[(Coord, i64)]) -> Self {
        let mut quad = Self {
            pos,
            width,
            num_bots: 0,
        };
        quad.num_bots = nanobots.iter().filter(|bot| quad.intersects(bot)).count();
        quad
    }

    fn intersects(&self, (pos, r): &(Coord, i64)) -> bool {
        // half open interval [a, b) distance to x
        fn inverval_dist(x: i64, interval: (i64, i64)) -> i64 {
            if x < interval.0 {
                interval.0 - x
            } else if x >= interval.1 {
                x - interval.1 + 1
            } else {
                0
            }
        }

        let d = inverval_dist(pos.0, (self.pos.0, self.pos.0 + self.width))
            + inverval_dist(pos.1, (self.pos.1, self.pos.1 + self.width))
            + inverval_dist(pos.2, (self.pos.2, self.pos.2 + self.width));
        d <= *r
    }

    fn subdivide(&self, nanobots: &[(Coord, i64)]) -> [Quad; 8] {
        let w = self.width / 2;
        let Coord(x, y, z) = self.pos;
        let res = [
            Self::new(Coord(x, y, z), w, nanobots),
            Self::new(Coord(x + w, y, z), w, nanobots),
            Self::new(Coord(x + w, y + w, z), w, nanobots),
            Self::new(Coord(x + w, y, z + w), w, nanobots),
            Self::new(Coord(x + w, y + w, z + w), w, nanobots),
            Self::new(Coord(x, y + w, z), w, nanobots),
            Self::new(Coord(x, y + w, z + w), w, nanobots),
            Self::new(Coord(x, y, z + w), w, nanobots),
        ];
        assert!(res.iter().map(|q| q.num_bots).sum::<usize>() >= self.num_bots);
        res
    }
}

impl Ord for Quad {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = self
            .num_bots
            .cmp(&other.num_bots)
            .reverse()
            .then(self.pos.norm().cmp(&other.pos.norm()))
            .then(self.width.cmp(&other.width));
        res.reverse() // reverse since it is used in max-heap
    }
}

impl PartialOrd for Quad {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn best_pos_dist(nanobots: &[(Coord, i64)]) -> i64 {
    let (min, max) = nanobots.iter().fold(
        (
            Coord(i64::MAX, i64::MAX, i64::MAX),
            Coord(i64::MIN, i64::MIN, i64::MIN),
        ),
        |(min, max), (pos, r)| {
            let corner_0 = Coord(pos.0 - r, pos.1, pos.2);
            let corner_1 = Coord(pos.0, pos.1 - r, pos.2);
            let corner_2 = Coord(pos.0, pos.1, pos.2 - r);
            (
                min.min_coords(&corner_0)
                    .min_coords(&corner_1)
                    .min_coords(&corner_2),
                max.max_coords(&corner_0)
                    .max_coords(&corner_1)
                    .max_coords(&corner_2),
            )
        },
    );

    let width = (max.0 - min.0).max(max.1 - min.1).max(max.2 - min.2).abs() as usize + 1;
    let width = (2 << (width as f64).log2().ceil() as usize) as i64;
    let root = Quad {
        pos: min,
        width,
        num_bots: nanobots.len(),
    };

    let mut pq = BinaryHeap::new();
    pq.push(root);

    while let Some(quad) = pq.pop() {
        if quad.width == 1 {
            return quad.pos.norm();
        }
        pq.extend(quad.subdivide(&nanobots).iter().cloned());
    }
    0
}

pub fn solve(input: &str) -> (usize, i64) {
    let nanobots = parse(input);
    // assert_eq!(85761543, p2);
    (
        num_in_range_of_strongest(&nanobots),
        best_pos_dist(&nanobots),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        const INPUT: &str = r#"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"#;
        let nanobots = parse(INPUT);
        assert_eq!(num_in_range_of_strongest(&nanobots), 7);
    }

    #[test]
    fn test_best_pos_dist() {
        const INPUT: &str = r#"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"#;
        let nanobots = parse(INPUT);
        assert_eq!(best_pos_dist(&nanobots), 36);
    }

    #[test]
    fn test_intersects() {
        let q = Quad {
            pos: Coord(10, 10, 10),
            width: 1,
            num_bots: 0,
        };
        assert!(!q.intersects(&(Coord(12, 14, 12), 2)));
    }
}
