use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

const ZERO: Point = Point { x: 0, y: 0 };

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match manhattan_distance(self, &ZERO).cmp(&manhattan_distance(other, &ZERO)) {
            Ordering::Equal => None,
            other => Some(other),
        }
    }
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

pub fn solve(input: &str) -> usize {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_largest_area() {
        const INPUT: &str = r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"#;
        assert_eq!(solve(INPUT), 17);
    }
}
