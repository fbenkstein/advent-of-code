fn parse_coords(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|x| {
            let coords: Vec<usize> = x
                .split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect();
            (coords[0], coords[1])
        }).collect()
}

fn manhattan_dist(ax: usize, ay: usize, bx: usize, by: usize) -> usize {
    ((ax as i32 - bx as i32).abs() + (ay as i32 - by as i32).abs()) as usize
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> usize {
    let coords = parse_coords(input);

    let width = *coords.iter().map(|(x, _y)| x).max().unwrap() + 1;
    let height = *coords.iter().map(|(_x, y)| y).max().unwrap() + 1;

    let mut map: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    let mut count: Vec<usize> = vec![0; coords.len()];
    let mut finite: Vec<bool> = vec![true; coords.len()];

    for y in 0..height {
        for x in 0..width {
            let mut min_dist: usize = width + height + 1;
            let mut min_i: Vec<usize> = Vec::new();

            for i in 0..coords.len() {
                let (ax, ay) = coords[i];
                let dist = manhattan_dist(x, y, ax, ay);
                if dist < min_dist {
                    min_dist = dist;
                    min_i = Vec::new();
                }
                if dist <= min_dist {
                    min_i.push(i);
                }
            }

            if min_i.len() == 1 {
                let i = min_i[0];
                map[y][x] = Some(i);
                count[i] += 1;

                if x == 0 || y == 0 || x + 1 == width || y + 1 == height {
                    finite[i] = false;
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(i) = map[y][x] {
                print!("{}", i);
            } else {
                print!(".");
            }
        }
        println!("");
    }

    (0..coords.len())
        .filter(|i| finite[*i])
        .map(|i| count[i])
        .max()
        .unwrap()
}

pub fn solve(input: &str) -> usize {
    let coords = parse_coords(input);

    let width = *coords.iter().map(|(x, _y)| x).max().unwrap() + 1;
    let height = *coords.iter().map(|(_x, y)| y).max().unwrap() + 1;

    let mut safe_area: usize = 0;

    for y in 0..height {
        for x in 0..width {
            let total_dist: usize = coords
                .iter()
                .map(|(ax, ay)| manhattan_dist(x, *ax, y, *ay))
                .sum();

            if total_dist < 10000 {
                safe_area += 1;
            }
        }
    }
    safe_area
}
