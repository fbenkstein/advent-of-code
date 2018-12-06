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

fn solve1(points: &[Point], width: i32, height: i32) -> usize {
    let mut areas = vec![Some(0); points.len()];
    for x in 0..width {
        for y in 0..height {
            let origin = Point { x, y };
            let (_, min_pos) = points.iter().enumerate().fold(
                (i32::max_value(), None),
                |(min_d, min_pos), (pos, pt)| {
                    let d = manhattan_distance(pt, &origin);
                    if d < min_d {
                        (d, Some(pos))
                    } else if d == min_d {
                        (min_d, None)
                    } else {
                        (min_d, min_pos)
                    }
                },
            );
            if let Some(pos) = min_pos {
                if is_edge_point(x, y, width, height) {
                    areas[pos] = None;
                } else {
                    areas[pos].as_mut().map(|a| *a += 1);
                }
            }
        }
    }
    areas.into_iter().filter_map(|x| x).max().unwrap()
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

pub fn solve(input: &str) -> (usize, usize) {
    let (points, width, height) = parse(input);
    (
        solve1(&points[..], width, height),
        solve2(&points[..], width, height, 10_000),
    )
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
