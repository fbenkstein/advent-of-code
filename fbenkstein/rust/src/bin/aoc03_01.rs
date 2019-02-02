extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate itertools;

use std::collections::BTreeMap;
use std::str::FromStr;

use itertools::Itertools;

use regex::Regex;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Cell {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rect {
    fn cells(&self) -> impl Iterator<Item = Cell> {
        (self.x..self.x + self.width)
            .cartesian_product(self.y..self.y + self.height)
            .map(|(x, y)| Cell { x, y })
    }
}

impl FromStr for Rect {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("^(?P<x>[0-9]+),(?P<y>[0-9]+): (?P<width>[0-9]+)x(?P<height>[0-9]+)$")
                    .expect("invalid regex");
        }
        let captures = RE
            .captures(s)
            .ok_or_else(|| "invalid rect string".to_string())?;
        let parse_field = |name| {
            captures
                .name(name)
                .unwrap()
                .as_str()
                .parse()
                .map_err(|_| format!("invalid {}", name))
        };
        let x: u32 = parse_field("x")?;
        let y: u32 = parse_field("y")?;
        let width = parse_field("width")?;
        let height = parse_field("height")?;
        Ok(Rect {
            x,
            y,
            width,
            height,
        })
    }
}

#[derive(Debug)]
struct Claim {
    id: u32,
    rect: Rect,
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("^#(?P<id>[0-9]+) @ (?P<rect>.*)$").expect("invalid regex");
        }
        let captures = RE
            .captures(s)
            .ok_or_else(|| "invalid claim string".to_string())?;
        let id_str = captures.name("id").unwrap().as_str();
        let id: u32 = id_str.parse().map_err(|_| "invalid id".to_string())?;
        let rect_str = captures.name("rect").unwrap().as_str();
        let rect: Rect = rect_str.parse()?;
        Ok(Claim { id, rect })
    }
}

fn main() {
    let data = include_str!("input_03.txt");
    let mut covered_cell_counts = BTreeMap::<Cell, usize>::new();

    for line in data.lines() {
        let claim: Claim = line.parse().unwrap();
        for cell in claim.rect.cells() {
            let count = covered_cell_counts.entry(cell).or_insert(0);
            *count += 1;
        }
    }

    println!(
        "{}",
        covered_cell_counts.values().filter(|x| **x > 1).count()
    );
}
