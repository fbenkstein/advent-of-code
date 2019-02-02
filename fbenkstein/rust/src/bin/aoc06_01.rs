#[macro_use]
extern crate text_io;
extern crate itertools;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    One,
    Two,
    Three,
    Four,
}

impl Point {
    fn distance_to(&self, other: Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    fn direction_to(&self, other: Self) -> Option<Direction> {
        if *self == other {
            None
        } else {
            Some(match (other.x > self.x, other.y > self.y) {
                (true, true) => Direction::One,
                (false, true) => Direction::Two,
                (false, false) => Direction::Three,
                (true, false) => Direction::Four,
            })
        }
    }

    fn circle(&self, radius: u32) -> impl Iterator<Item = Point> {
        let base = *self;
        let radius = radius as i32;
        std::iter::empty()
            .chain((0..radius).map(move |d| (radius - d, d)))
            .chain((0..radius).map(move |d| (-d, radius - d)))
            .chain((0..radius).map(move |d| (d - radius, -d)))
            .chain((0..radius).map(move |d| (d, d - radius)))
            .map(move |(xd, yd)| Self {
                x: base.x + xd,
                y: base.y + yd,
            })
    }
}

#[cfg(test)]
mod test_location {
    use super::Point;
    use itertools::Itertools;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_distance_to() {
        let l = |x, y| Point { x, y };
        assert_eq!(l(0, 0).distance_to(l(0, 0)), 0);
        assert_eq!(l(0, 0).distance_to(l(7, 0)), 7);
        assert_eq!(l(7, 0).distance_to(l(0, 0)), 7);
        assert_eq!(l(0, 0).distance_to(l(7, 8)), 15);
        assert_eq!(l(7, 8).distance_to(l(0, 0)), 15);
        assert_eq!(l(7, 0).distance_to(l(0, 8)), 15);
    }

    #[test]
    fn test_circle() {
        let l = |x, y| Point { x, y };
        fn h<T: IntoIterator<Item = Point>>(iter: T) -> HashSet<Point> {
            HashSet::from_iter(iter)
        }
        assert_eq!(h(l(0, 0).circle(0)), HashSet::new());
        assert_eq!(
            h(l(0, 0).circle(1)),
            h(vec![l(1, 0), l(-1, 0), l(0, 1), l(0, -1)]),
        );
        assert_eq!(
            h(l(0, 0).circle(2)),
            h((-2..=2)
                .cartesian_product(-2..=2)
                .map(|(x, y)| l(x, y))
                .filter(|loc| loc.distance_to(l(0, 0)) == 2))
        );
    }
}

fn main() {
    let input = include_str!("input_06.txt").lines().map(|s| {
        let x: i32;
        let y: i32;
        scan!(s.bytes() => "{}, {}", x, y);
        Point { x, y }
    });
    let points: HashSet<_> = input.clone().collect();
    // Border points are those points that do not have points in every
    // direction. These will have infinite areas.
    let border_points: HashSet<_> = input
        .clone()
        .filter(|point| {
            input
                .clone()
                .filter_map(|other_point| point.direction_to(other_point))
                .collect::<HashSet<_>>()
                .len()
                != 4
        })
        .collect();
    // choose as starting point a point that has minimal average distance to all
    // other points
    let start_point = input
        .clone()
        .min_by_key(|point| {
            input
                .clone()
                .map(|other_point| point.distance_to(other_point))
                .sum::<u32>()
        })
        .unwrap();
    let mut area_sizes = HashMap::<Point, usize>::new();
    let n = points.len();
    let mut seen_points = 1;

    // visit all points in concentric circles around the starting point
    for visit_radius in 1.. {
        if seen_points == n {
            break;
        }

        for visit_point in start_point.circle(visit_radius) {
            if points.contains(&visit_point) {
                seen_points += 1;
                continue;
            }

            let points_by_distance = points
                .iter()
                .sorted_by_key(|point| visit_point.distance_to(**point));
            let closest_point = points_by_distance[0];
            let next_closest_point = points_by_distance[1];

            if closest_point.distance_to(visit_point) != next_closest_point.distance_to(visit_point)
                && !border_points.contains(closest_point)
            {
                *area_sizes.entry(*closest_point).or_insert(1) += 1;
            }
        }
    }

    println!("{:?}", area_sizes.values().max().unwrap());
}
