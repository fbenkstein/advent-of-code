use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;

use log::debug;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
pub enum Class {
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
pub struct Unit {
    pub class: Class,
    x: usize,
    y: usize,
    hp: isize,
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

#[derive(Debug, Clone)]
pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    units: Vec<Unit>,
    turn: usize,
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
        let mut units = Vec::new();

        for (y, line) in contents.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                match tile {
                    'E' => {
                        units.push(Unit {
                            class: Class::Elf,
                            x: x,
                            y: y,
                            hp: 200,
                        });
                        tiles.push(Tile::Open);
                    }
                    'G' => {
                        units.push(Unit {
                            class: Class::Goblin,
                            x: x,
                            y: y,
                            hp: 200,
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
            turn: 0,
        })
    }
}

impl Unit {
    fn points_around(x: usize, y: usize) -> [(usize, usize); 4] {
        [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
    }

    /// Determines all of the targets that are in range of it by being immediately adjacent to it
    /// then gets the one that has the fewest hit points (hp) and first in readability order
    fn has_target_in_range(&self, units: &Vec<Unit>) -> Option<(usize, Unit)> {
        let adjacent_tiles: Vec<(usize, usize)> = Unit::points_around(self.x, self.y).to_vec();
        units
            .iter()
            .enumerate()
            .filter_map(|(i, &u)| {
                if adjacent_tiles.contains(&(u.x, u.y)) && u.class != self.class {
                    return Some((i, u.clone()));
                } else {
                    None
                }
            })
            .min_by_key(|(_, u)| (u.hp, u.y, u.x))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  ")?;
        for i in 0..self.width {
            write!(f, "{}", i)?;
        }
        writeln!(f, "")?;
        for y in 0..self.height {
            write!(f, "{} ", y)?;
            for x in 0..self.width {
                if let Some(unit) = self.units.iter().find(|p| p.x == x && p.y == y) {
                    write!(f, "{}", unit.class)?;
                } else {
                    write!(f, "{}", self.tiles[x + y * self.width])?;
                }
            }
            let units: BTreeSet<&Unit> = self.units.iter().filter(|u| u.y == y).collect();
            for unit in units {
                write!(f, " {}({})", unit.class, unit.hp)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Turn {
    GameOver,
    Attacked(usize),
    Moved,
    Victory(Class),
    Error,
}

impl Board {
    pub fn next_turn(&mut self) -> Option<isize> {
        let mut dead_units = vec![];
        println!("Start round {}:", self.turn + 1);
        for i in 0..self.units.len() {
            let last_move = self.choose_turn(i, &dead_units);
            match last_move {
                Turn::Attacked(ennemy_idx) => {
                    let ennemy = self.units.get_mut(ennemy_idx).expect("cannot index ennemy");
                    ennemy.hp -= 3;
                    if ennemy.hp <= 0 {
                        self.units.remove(i);
                    }
                }
                Turn::Victory(class) => {
                    println!("Victory from {}", class);
                    return Some(
                        self.turn as isize * self.units.iter().map(|&u| u.hp).sum::<isize>(),
                    );
                }
                _ => continue,
            };
        }
        for dead_unit in dead_units.iter() {
            self.units.remove(*dead_unit);
        }
        self.turn += 1;
        println!("{}", self);
        self.units.sort();
        None
    }

    /// Choose the step and perform attack if relevant
    /// returns the index of the ennemy that got killed.
    pub fn choose_turn(&mut self, idx: usize, dead_units: &Vec<usize>) -> Turn {
        let mut current_units = self
            .units
            .iter()
            .enumerate()
            .filter(|(i, _)| !dead_units.contains(i))
            .map(|(_, &u)| u)
            .collect();
        let unit = self.units.get_mut(idx);
        let unit = unit.unwrap();
        let current_tiles = self.tiles.clone();
        let width = self.width.clone();

        if let Some((ennemy_idx, ennemy_unit)) = unit.has_target_in_range(&mut current_units) {
            println!(
                "{:?} is already ready to attack {:?}",
                unit, current_units[ennemy_idx]
            );
            return Turn::Attacked(ennemy_idx);
        }

        let find_reachable_tiles = |(x, y)| -> Vec<(usize, usize)> {
            Unit::points_around(x, y)
                .into_iter()
                .filter(|(x, y)| {
                    current_tiles[x + y * width] == Tile::Open
                        && !current_units.iter().any(|u| u.x == *x && u.y == *y)
                })
                .map(|&(x, y)| (x, y))
                .collect()
        };
        debug!("{:?}", unit);

        // find all tiles each target can reach
        let in_range: HashSet<(usize, usize)> = current_units
            .iter()
            .cloned()
            .filter(|other: &Unit| other != unit && other.class != unit.class)
            .flat_map(|unit| find_reachable_tiles((unit.x, unit.y)))
            .collect();
        debug!("In range: {:?}", in_range);
        if in_range.len() == 0 {
            return Turn::Victory(unit.class);
        }

        // find all reachable tiles for this unit
        let mut reachable_tiles = HashSet::new();
        let mut queue = VecDeque::new();
        reachable_tiles.insert((unit.x, unit.y));
        queue.push_back((unit.x, unit.y));
        while let Some(point) = queue.pop_front() {
            for reachable in find_reachable_tiles(point).iter() {
                if reachable_tiles.insert(*reachable) {
                    queue.push_back(*reachable);
                }
            }
        }

        // intersect with the ennemy next tiles
        // find nearest
        // TODO: is this where it's better to use a priority queue? (i.e. do everything in one operation)
        let mut nearest_tiles: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
        for (x, y) in in_range.intersection(&reachable_tiles) {
            let distance =
                (unit.x as isize - *x as isize).abs() + (unit.y as isize - *y as isize).abs();
            if distance > 0 {
                nearest_tiles
                    .entry(distance as usize)
                    .or_default()
                    .push((*x, *y));
            }
        }

        debug!("Nearest tiles are {:?}", nearest_tiles);

        if let Some(chosen_tiles) = nearest_tiles.iter().min() {
            let chosen_tile = chosen_tiles.1.iter().min_by_key(|(x, y)| (y, x)).unwrap();
            debug!("Target tile: {:?}", chosen_tile);

            // all reachable points around me
            // cheapest move is the one with the lowest distance AND first in readability order
            let mut around_unit = HashSet::new();
            let pts = Unit::points_around(unit.x, unit.y);
            around_unit.extend(pts.into_iter());
            let cheapest_move = reachable_tiles
                .intersection(&around_unit)
                .min_by_key(|(x, y)| {
                    let distance = (chosen_tile.0 as isize - *x as isize).abs()
                        + (chosen_tile.1 as isize - *y as isize).abs();
                    (distance, y, x)
                })
                .expect("nothing's cheap??");
            debug!("Chosen tile to move to: {:?}", cheapest_move);

            unit.x = cheapest_move.0;
            unit.y = cheapest_move.1;
            // we replace the unit we just updated in the current_units array (ewww)
            current_units[idx] = *unit;
        }

        if let Some((ennemy_idx, _)) = unit.has_target_in_range(&current_units) {
            println!(
                "{:?} is ready to attack {:?}",
                unit, current_units[ennemy_idx]
            );
            return Turn::Attacked(ennemy_idx);
        }

        return Turn::Moved;
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
