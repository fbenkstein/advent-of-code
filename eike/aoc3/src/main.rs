#[macro_use]
extern crate text_io;

use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Rectangle {
    id: i32,
    x: usize,
    y: usize,
    xl: usize,
    yl: usize,
}

fn parse_claims(lines: &Vec<String>) -> Vec<Rectangle> {
    let mut claims: Vec<Rectangle> = Vec::new();
    for line in lines {
        let mut claim = Rectangle {
            id: 0,
            x: 0,
            y: 0,
            xl: 0,
            yl: 0,
        };
        scan!(line.bytes() => "#{} @ {},{}: {}x{}", claim.id, claim.x, claim.y, claim.xl, claim.yl);
        claims.push(claim);
    }
    claims
}

#[derive(Debug, Clone)]
struct Canvas {
    data: Vec<Vec<i32>>,
}

impl Canvas {
    fn new(size: (usize, usize)) -> Canvas {
        let mut canvas: Vec<Vec<i32>> = Vec::new();
        for _ in 0..size.0 {
            canvas.push(vec![0; size.1]);
        }
        Canvas { data: canvas }
    }

    fn add(&mut self, rect: &Rectangle) {
        for xi in rect.x..(rect.x + rect.xl) {
            for yi in rect.y..(rect.y + rect.yl) {
                self.data[xi][yi] += 1;
            }
        }
    }

    fn bounded_sum(&self, rect: &Rectangle) -> i32 {
        let mut sum = 0;
        for xi in rect.x..(rect.x + rect.xl) {
            for yi in rect.y..(rect.y + rect.yl) {
                sum += self.data[xi][yi] ;
            }
        }
        sum
    }

    fn map(&self, transform: fn(i32) -> i32) -> Canvas {
        let mut result = self.clone();
        result.data.iter_mut().for_each(|row| row.iter_mut().for_each(|value| *value = transform(*value)));
        result
    }

    fn sum(&self) -> i32 {
        self.data.iter().map(|row| row.iter().sum()).fold(0i32, |sum, value: i32| sum + value )
    }
}

fn main() {
    // io
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let claims = parse_claims(&lines);

    // --- part one ---
    let mut canvas = Canvas::new((1000, 1000));
    for claim in &claims {
        canvas.add(&claim);
    }

    let overlap_canvas = canvas.map(|value| { if value > 1 { 1 } else { 0 }});
    println!("Overlapping area is: {}", overlap_canvas.sum());

    // --- part two ---
    for claim in &claims {
        if overlap_canvas.bounded_sum(&claim) == 0 {
            println!("Claim #{} has no overlap with any other claim", claim.id);
        }
    }
}
