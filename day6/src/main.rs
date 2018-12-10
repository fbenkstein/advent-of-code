use std::fs::File;
use std::io::prelude::*;

use itertools::iproduct;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn is_edge_point(x: i32, y: i32, width: i32, height: i32) -> bool {
    x == 0 || y == 0 || x + 1 == width || y + 1 == height
}

fn area(grid: &[Option<Point>], width: i32, height: i32, origin: &Point) -> Option<usize> {
    let mut counter = 0;
    for x in 0..width {
        for y in 0..height {
            if let Some(pt) = &grid[(x + width * y) as usize] {
                if pt == origin {
                    if is_edge_point(x, y, width, height) {
                        return None;
                    }
                    counter += 1;
                }
            }
        }
    }
    Some(counter)
}

fn solve1(points: &[Point], width: i32, height: i32) -> usize {
    let mut grid: Vec<Option<Point>> = vec![None; (width * height) as usize];

    for x in 0..width {
        for y in 0..height {
            let origin = Point { x, y };
            let (_, min_pt) = points.iter().fold(
                (
                    i32::max_value(),
                    Some(Point {
                        x: width,
                        y: height,
                    }),
                ),
                |(min_d, min_pt), pt| {
                    let d = manhattan_distance(pt, &origin);
                    if d < min_d {
                        (d, Some(*pt))
                    } else if d == min_d {
                        (min_d, None)
                    } else {
                        (min_d, min_pt)
                    }
                },
            );
            grid[(x + width * y) as usize] = min_pt;
        }
    }

    points
        .iter()
        .filter_map(|pt| area(&grid[..], width, height, pt))
        .max()
        .unwrap()
}

fn solve2(points: &[Point], width: i32, height: i32, max_distance: i32) -> usize {
    iproduct!((0..width), (0..height))
        .filter_map(|(x, y)| {
            let total_distance: i32 = points
                .iter()
                .map(|pt| manhattan_distance(&Point { x, y }, pt))
                .sum();
            if total_distance < max_distance {
                Some(1)
            } else {
                None
            }
        })
        .sum()
}

fn parse(input: &str) -> (Vec<Point>, i32, i32) {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let mut it = line.split(", ");
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            Point { x, y }
        })
        .collect();
    let width = points.iter().map(|pt| pt.x).max().unwrap() + 1;
    let height = points.iter().map(|pt| pt.y).max().unwrap() + 1;
    (points, width, height)
}

pub fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file
        .read_to_string(&mut contents)
        .expect("could not read file");
    let (points, width, height) = parse(&contents);
    println!("{:?}", solve1(&points[..], width, height));
    println!("{:?}", solve2(&points[..], width, height, 10_000));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"#;

    #[test]
    fn test_solve1() {
        let (points, width, height) = parse(INPUT);
        assert_eq!(solve1(&points[..], width, height), 17);
    }

    #[test]
    fn test_solve2() {
        let (points, width, height) = parse(INPUT);
        assert_eq!(solve2(&points[..], width, height, 32), 16);
    }
}
