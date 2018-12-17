use itertools::Itertools;
use text_io::{scan, try_scan};

use std::fmt::{self, Write};
use std::mem;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Sand,
    Clay,
    Water,
    Flow,
    Faucet,
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Sand => '.',
            Tile::Clay => '#',
            Tile::Water => '~',
            Tile::Faucet => '+',
            Tile::Flow => '|',
        }
    }
}

struct Ground {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    cursors: Vec<(usize, usize)>,
}

impl Ground {
    fn flow(&mut self) -> bool {
        let mut stack: Vec<(usize, usize)> = Vec::new();
        mem::swap(&mut self.cursors, &mut stack);

        while let Some((x, y)) = stack.pop() {
            if y + 1 >= self.height {
                continue;
            }

            let tile = self[(x, y)];
            if tile != Tile::Flow && tile != Tile::Faucet {
                continue;
            }

            match self[(x, y + 1)] {
                Tile::Sand => {
                    self[(x, y + 1)] = Tile::Flow;
                    self.cursors.push((x, y + 1));
                }
                Tile::Clay | Tile::Water => {
                    if self[(x - 1, y)] == Tile::Sand {
                        self[(x - 1, y)] = Tile::Flow;
                        self.cursors.push((x - 1, y));
                    }

                    if self[(x + 1, y)] == Tile::Sand {
                        self[(x + 1, y)] = Tile::Flow;
                        self.cursors.push((x + 1, y));
                    }

                    if self[(x - 1, y)] == Tile::Clay
                        || self[(x - 1, y)] == Tile::Flow
                        || self[(x + 1, y)] == Tile::Clay
                        || self[(x + 1, y)] == Tile::Flow
                    {
                        let dneg = (0..x)
                            .rev()
                            .take_while(|&x| self[(x, y)] == Tile::Flow)
                            .count();
                        let dpos = (x + 1..self.width)
                            .take_while(|&x| self[(x, y)] == Tile::Flow)
                            .count();
                        if x >= dneg + 1
                            && self[(x - dneg - 1, y)] == Tile::Clay
                            && self[(x + dpos + 1, y)] == Tile::Clay
                        {
                            for x in x - dneg..=x + dpos {
                                self[(x, y)] = Tile::Water;
                                if self[(x, y - 1)] == Tile::Flow {
                                    self.cursors.push((x, y - 1));
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        !self.cursors.is_empty()
    }
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

    let (mut offset_x, max_x) = clay_tiles
        .iter()
        .map(|(x, _)| *x)
        .minmax()
        .into_option()
        .unwrap();
    let (mut offset_y, max_y) = clay_tiles
        .iter()
        .map(|(_, y)| *y)
        .minmax()
        .into_option()
        .unwrap();
    offset_x -= 1;
    offset_y -= 1;

    let width = max_x + 2 - offset_x;
    let height = max_y + 1 - offset_y;
    let tiles = vec![Tile::Sand; width * height];
    let mut ground = Ground {
        width,
        height,
        tiles,
        cursors: Vec::new(),
    };

    for clay_pos in clay_tiles {
        ground[(clay_pos.0 - offset_x, clay_pos.1 - offset_y)] = Tile::Clay;
    }
    // add sprinkler
    ground[(500 - offset_x, 0)] = Tile::Faucet;
    ground.cursors.push((500 - offset_x, 0));

    ground
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut ground = parse(input);
    while ground.flow() {}
    println!("{}", ground);
    let part1 = ground
        .tiles
        .iter()
        .filter(|&&t| t == Tile::Water || t == Tile::Flow)
        .count();
    let part2 = ground.tiles.iter().filter(|&&t| t == Tile::Water).count();
    (part1, part2)
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
