use std::f32;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::str;
use text_io::{scan, try_scan};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    const MIN: Self = Self {
        x: isize::min_value(),
        y: isize::min_value(),
    };

    const MAX: Self = Self {
        x: isize::max_value(),
        y: isize::max_value(),
    };

    fn min(self, other: Point) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(self, other: Point) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl Add<Vector> for &Point {
    type Output = Point;
    fn add(self, v: Vector) -> Self::Output {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, pt: Point) -> Self::Output {
        Vector {
            x: self.x - pt.x,
            y: self.y - pt.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }
}

impl Mul<&Vector> for isize {
    type Output = Vector;
    fn mul(self, v: &Vector) -> Self::Output {
        Vector {
            x: self * v.x,
            y: self * v.y,
        }
    }
}

fn parse(input: &str) -> Vec<(Point, Vector)> {
    input
        .lines()
        .map(|line| {
            let x;
            let y;
            let vx;
            let vy;
            scan!(line.bytes().filter(|&x| x != ' ' as u8) =>
                "position=<{},{}>velocity=<{},{}>", x, y, vx, vy);
            (Point { x, y }, Vector { x: vx, y: vy })
        })
        .collect()
}

pub fn solve(input: &str) -> isize {
    let configuration = parse(input);

    let mut prev_weighted_dist = f32::MAX;
    let mut t = 0;

    let t = loop {
        let min = configuration
            .iter()
            .map(|(pt, v)| pt + t * v)
            .fold(Point::MAX, Point::min);
        let weighted_dist = configuration
            .iter()
            .map(|(pt, v)| (pt + t * v - min).length())
            .sum::<f32>()
            / configuration.len() as f32;

        if prev_weighted_dist < weighted_dist {
            break t - 1;
        }

        prev_weighted_dist = weighted_dist;
        t += 1;
    };

    let (min, max) = configuration
        .iter()
        .fold((Point::MAX, Point::MIN), |(min, max), (pt, v)| {
            (min.min(pt + t * v), max.max(pt + t * v))
        });

    let w = (max.x - min.x + 1) as usize;
    let h = (max.y - min.y + 1) as usize;

    let mut fields = Vec::with_capacity(w * h);
    fields.resize((w * h) as usize, ' ' as u8);
    for (pt, v) in &configuration {
        let pt = pt + t * v - min;
        let idx = pt.x + (w as isize) * pt.y;
        fields[idx as usize] = '*' as u8;
    }

    let mut res = String::new();
    for y in 0..h {
        let line = str::from_utf8(&fields[w * y..w * (y + 1)]).unwrap();
        res += line;
        res.push('\n');
    }
    println!("{}", res);
    t
}
