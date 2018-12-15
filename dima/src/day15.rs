use std::fmt::{self, Write};
use std::ops::Index;

#[derive(Debug)]
enum Field {
    Wall,
    Open,
    Goblin(usize),
    Elf(usize),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Field::Wall => f.write_char('#'),
            Field::Open => f.write_char('.'),
            Field::Elf(_) => f.write_char('E'),
            Field::Goblin(_) => f.write_char('G'),
        }
    }
}

#[derive(Debug)]
struct Map {
    fields: Vec<Field>,
    width: usize,
    height: usize,
}

impl Index<(usize, usize)> for Map {
    type Output = Field;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.fields[x + self.width * y]
    }
}

#[derive(Debug)]
enum Race {
    Elf,
    Goblin,
}

#[derive(Debug)]
struct Npc {
    pos: (usize, usize),
    hit_points: i32,
    race: Race,
}

impl Npc {
    const ATTACK_POWER: i32 = 3;
    const INITIAL_HIT_POINTS: i32 = 200;
}

impl fmt::Display for Npc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.race {
            Race::Elf => write!(f, "E({})", self.hit_points),
            Race::Goblin => write!(f, "G({})", self.hit_points),
        }
    }
}

#[derive(Debug)]
struct Game {
    map: Map,
    npcs: Vec<Npc>,
}

impl From<&str> for Game {
    fn from(input: &str) -> Game {
        let mut fields = Vec::new();
        let mut npcs = Vec::new();
        let mut x = 0;
        let mut y = 0;

        for line in input.lines() {
            x = 0;
            for c in line.chars() {
                let field = match c {
                    '#' => Field::Wall,
                    '.' => Field::Open,
                    'E' => {
                        let field = Field::Elf(npcs.len());
                        npcs.push(Npc {
                            pos: (x, y),
                            hit_points: Npc::INITIAL_HIT_POINTS,
                            race: Race::Elf,
                        });
                        field
                    }
                    'G' => {
                        let field = Field::Goblin(npcs.len());
                        npcs.push(Npc {
                            pos: (x, y),
                            hit_points: Npc::INITIAL_HIT_POINTS,
                            race: Race::Goblin,
                        });
                        field
                    }
                    _ => panic!("invalid input"),
                };
                fields.push(field);
                x += 1;
            }
            y += 1;
        }

        let map = Map {
            fields,
            width: x,
            height: y,
        };
        println!("{:?}", map);
        Self { map, npcs }
    }
}

// impl FromStr for Game {
//     type Err = ();
//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         Err(())
//     }
// }

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut npcs_per_line = Vec::new();
        for y in 0..self.map.height {
            npcs_per_line.clear();

            for x in 0..self.map.width {
                let field = &self.map[(x, y)];
                match field {
                    Field::Goblin(idx) | Field::Elf(idx) => npcs_per_line.push(&self.npcs[*idx]),
                    _ => (),
                };
                write!(f, "{}", field)?;
            }

            if !npcs_per_line.is_empty() {
                write!(f, "   ")?;
            }
            writeln!(
                f,
                "{}",
                npcs_per_line
                    .iter()
                    .map(|npc| format!("{}", npc))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> usize {
    let game = Game::from(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#######
#.G.E.#
#E.G.E#
#.G.E.#
#######"#;

    #[test]
    fn test_parse() {
        let game = Game::from(INPUT);
        const EXPECTED: &str = r#"#######
#.G.E.#   G(200), E(200)
#E.G.E#   E(200), G(200), E(200)
#.G.E.#   G(200), E(200)
#######
"#;
        assert_eq!(format!("{}", game), EXPECTED);
    }
}
