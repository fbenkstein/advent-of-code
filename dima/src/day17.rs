use std::fmt::{self, Write};
use std::ops::{Index, IndexMut};
use text_io::{scan, try_scan};

#[derive(Clone, Copy)]
enum Tile {
    Sand,
    Clay,
    Water,
    Faucet,
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Sand => '.',
            Tile::Clay => '#',
            Tile::Water => '~',
            Tile::Faucet => '+',
        }
    }
}

struct Ground {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Index<(usize, usize)> for Ground {
    type Output = Tile;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Ground {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.tiles[x + y * self.width]
    }
}

impl fmt::Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_char(self[(x, y)].into())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Ground {
    let clay_tiles: Vec<_> = input
        .lines()
        .flat_map(|line| {
            let a: char;
            let _b: char;
            let a_value: usize;
            let b_from: usize;
            let b_to: usize;
            scan!(line.bytes() => "{}={}, {}={}..{}", a, a_value, _b, b_from, b_to);
            let tiles: Vec<_> = match a {
                'x' => (b_from..=b_to).map(|y| (a_value, y)).collect(),
                'y' => (b_from..=b_to).map(|x| (x, a_value)).collect(),
                _ => panic!(),
            };
            tiles
        })
        .collect();

    let offset = clay_tiles.iter().map(|(x, _)| x).min().unwrap() - 1;
    let width = clay_tiles.iter().map(|(x, _)| x).max().unwrap() + 2 - offset;
    let height = clay_tiles.iter().map(|(_, y)| y).max().unwrap() + 1;
    let tiles = vec![Tile::Sand; width * height];
    let mut ground = Ground {
        width,
        height,
        tiles,
    };

    for clay_pos in clay_tiles {
        ground[(clay_pos.0 - offset, clay_pos.1)] = Tile::Clay;
    }
    // add sprinkler
    ground[(500 - offset, 0)] = Tile::Faucet;

    ground
}

pub fn solve(input: &str) -> usize {
    let ground = parse(input);
    println!("{}", ground);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        const INPUT: &str = r#"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"#;

        let s = format!("{}", parse(INPUT));
        println!("{}", s);
        assert_eq!(
            s,
            r#"......+.......
............#.
.#..#.......#.
.#..#..#......
.#..#..#......
.#.....#......
.#.....#......
.#######......
..............
..............
....#.....#...
....#.....#...
....#.....#...
....#######...
"#
        );
    }
}
