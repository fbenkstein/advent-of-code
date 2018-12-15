use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Tile {
    Wall,
    Open,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Open => write!(f, "."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Class {
    Goblin,
    Elf,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Class::Goblin => write!(f, "G"),
            Class::Elf => write!(f, "E"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Unit {
    pub class: Class,
    x: usize,
    y: usize,
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Unit) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

#[derive(Debug)]
pub struct Board {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    units: BTreeSet<Unit>,
}

impl FromStr for Board {
    type Err = String;
    fn from_str(contents: &str) -> Result<Board, Self::Err> {
        let height = contents.lines().count();
        let width = contents
            .lines()
            .map(|line| line.len())
            .max()
            .expect("no max?!");
        let mut tiles = Vec::with_capacity(width * height);
        let mut units = BTreeSet::new();

        for (y, line) in contents.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                match tile {
                    'E' => {
                        units.insert(Unit {
                            class: Class::Elf,
                            x: x,
                            y: y,
                        });
                        tiles.push(Tile::Open);
                    }
                    'G' => {
                        units.insert(Unit {
                            class: Class::Goblin,
                            x: x,
                            y: y,
                        });
                        tiles.push(Tile::Open);
                    }
                    '#' => tiles.push(Tile::Wall),
                    '.' => {
                        tiles.push(Tile::Open);
                    }
                    _ => return Err(format!("Invalid tile: {}", tile)),
                }
            }
            for i in line.chars().count()..width {
                tiles.insert(i, Tile::Wall);
            }
        }

        Ok(Board {
            tiles: tiles,
            units: units,
            height: height,
            width: width,
        })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            writeln!(f, "")?;
            for x in 0..self.width {
                if let Some(unit) = self.units.iter().find(|p| p.x == x && p.y == y) {
                    write!(f, "{}", unit.class)?;
                } else {
                    write!(f, "{}", self.tiles[x + y * self.width])?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let board = Board::from_str(
            r#"
#######
#.G.E.#
#E.G.E#
#.G.E.#
#######"#,
        )
        .expect("could not parse board.");
        assert_eq!(
            board.units.iter().map(|u| u.class).collect::<Vec<Class>>(),
            &[
                Class::Goblin,
                Class::Elf,
                Class::Elf,
                Class::Goblin,
                Class::Elf,
                Class::Goblin,
                Class::Elf
            ]
        );
    }
}
