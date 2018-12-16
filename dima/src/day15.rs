use std::collections::{HashMap, VecDeque};
use std::fmt::{self, Write};
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
enum Field {
    Wall,
    Open,
    Npc(usize),
}

#[derive(Debug)]
struct Map {
    fields: Vec<Field>,
    width: usize,
    height: usize,
}

impl Map {
    fn neighbors<'a>(&'a self, (x, y): (usize, usize)) -> impl Iterator<Item = Position> + 'a {
        [(0, -1), (-1, 0), (1, 0), (0, 1)]
            .into_iter()
            .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
            .filter(move |&(x, y)| x < self.width && y < self.height)
    }

    fn open_neighbors<'a>(&'a self, pos: (usize, usize)) -> impl Iterator<Item = Position> + 'a {
        self.neighbors(pos).filter(move |&pos| match self[pos] {
            Field::Open => true,
            _ => false,
        })
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Field;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.fields[x + self.width * y]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.fields[x + self.width * y]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

type Position = (usize, usize);

impl Npc {
    const ATTACK_POWER: i32 = 3;
    const INITIAL_HIT_POINTS: i32 = 200;

    fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    fn hit(&mut self, power: i32) {
        self.hit_points -= power
    }
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

impl Game {
    fn do_turn(&mut self) -> bool {
        let npcs = self.alive_npcs_in_order();

        let all_elves = npcs.iter().all(|&idx| self.npcs[idx].race == Race::Elf);
        let all_goblins = npcs.iter().all(|&idx| self.npcs[idx].race == Race::Goblin);
        if all_elves || all_goblins {
            return false;
        }

        for &idx in &npcs {
            if self.npcs[idx].is_alive() {
                let race = self.npcs[idx].race;
                let any_enemy_alive = self
                    .npcs
                    .iter()
                    .any(|npc| npc.is_alive() && npc.race != race);
                if !any_enemy_alive {
                    self.remove_dead_bodies();
                    return false;
                }
                self.do_npc_turn(idx);
            }
        }

        self.remove_dead_bodies();
        true
    }

    fn do_npc_turn(&mut self, idx: usize) {
        if let Some(target_idx) = self.in_range_of_target(&self.npcs[idx]) {
            self.npcs[target_idx].hit(Npc::ATTACK_POWER);
        } else {
            self.do_npc_move(idx);
            if let Some(target_idx) = self.in_range_of_target(&self.npcs[idx]) {
                self.npcs[target_idx].hit(Npc::ATTACK_POWER);
            }
        }
    }

    fn in_range_of_target(&self, npc: &Npc) -> Option<usize> {
        self.map
            .neighbors(npc.pos)
            .filter_map(|pos| match &self.map[pos] {
                Field::Npc(idx) => {
                    let target_npc = &self.npcs[*idx];
                    if target_npc.is_alive() && npc.race != target_npc.race {
                        Some(*idx)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .min_by_key(|idx| {
                let npc = &self.npcs[*idx];
                (npc.hit_points, npc.pos.1, npc.pos.0)
            })
    }

    fn do_npc_move(&mut self, idx: usize) -> bool {
        let npc_pos = self.npcs[idx].pos;

        let npcs = self.alive_npcs_in_order();
        let targets = npcs
            .iter()
            .cloned()
            .filter(|&target_idx| self.npcs[target_idx].race != self.npcs[idx].race);
        let in_range = targets.flat_map(|idx| self.map.open_neighbors(self.npcs[idx].pos));
        let mut in_range: Vec<Position> = in_range.collect();

        if in_range.is_empty() {
            // nothing to do
            return false;
        }

        // BFS from npc to in range positions
        let mut distances = HashMap::new();
        distances.insert(npc_pos, 0);
        let mut queue = VecDeque::new();
        queue.push_back(npc_pos);
        while let Some(pos) = queue.pop_front() {
            for neighbor_pos in self.map.open_neighbors(pos) {
                if !distances.contains_key(&neighbor_pos) {
                    distances.insert(neighbor_pos, distances[&pos] + 1);
                    queue.push_back(neighbor_pos);
                }
            }
        }

        in_range.retain(|pos| distances.contains_key(pos));

        let reachable = in_range
            .into_iter()
            .filter(|pos| distances.contains_key(pos));
        let chosen = reachable.min_by_key(|pos| (distances[pos], pos.1, pos.0));
        let chosen = match chosen {
            Some(pos) => pos,
            None => return true, // nowhere to go
        };

        // BFS from chosen pos to npc
        queue.clear();
        queue.push_back(chosen);
        distances.clear();
        distances.insert(chosen, 0);
        while let Some(pos) = queue.pop_front() {
            if pos == npc_pos {
                // done
                break;
            }
            for neighbor_pos in self.map.open_neighbors(pos) {
                if !distances.contains_key(&neighbor_pos) {
                    distances.insert(neighbor_pos, distances[&pos] + 1);
                    queue.push_back(neighbor_pos);
                }
            }
        }

        // Next neightbor to move
        let move_to = self
            .map
            .neighbors(npc_pos)
            .filter_map(|pos| distances.get(&pos).map(|d| (d, pos.1, pos.0)))
            .min();
        let move_to_pos = match move_to {
            Some((_, y, x)) => (x, y),
            None => return false, // nowhere to move
        };

        // move
        // println!("Move {:?} -> {:?}", npc_pos, move_to_pos);

        self.map[npc_pos] = Field::Open;
        self.map[move_to_pos] = Field::Npc(idx);
        self.npcs[idx].pos = move_to_pos;

        true
    }

    fn alive_npcs_in_order(&self) -> Vec<usize> {
        let mut npcs: Vec<_> = (0..self.npcs.len())
            .filter(|&idx| self.npcs[idx].is_alive())
            .collect();
        npcs.sort_by_key(|&idx| (self.npcs[idx].pos.1, self.npcs[idx].pos.0));
        npcs
    }

    fn run(&mut self) -> (usize, usize) {
        println!("{}", self);

        let mut num_rounds = 0;
        loop {
            let cont = self.do_turn();
            println!("{}", self);
            if !cont {
                break;
            }
            num_rounds += 1;
        }

        let hit_points = self
            .npcs
            .iter()
            .filter(|npc| npc.is_alive())
            .map(|npc| npc.hit_points as usize)
            .sum::<usize>();

        (num_rounds, hit_points)
    }

    fn remove_dead_bodies(&mut self) {
        let dead_npcs = self
            .npcs
            .iter()
            .enumerate()
            .filter(|(_, npc)| !npc.is_alive());
        for (idx, npc) in dead_npcs {
            let not_yet_removed = match self.map[npc.pos] {
                Field::Npc(field_idx) if idx == field_idx => true,
                _ => false,
            };
            if not_yet_removed {
                self.map[npc.pos] = Field::Open;
            }
        }
    }
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
                        let field = Field::Npc(npcs.len());
                        npcs.push(Npc {
                            pos: (x, y),
                            hit_points: Npc::INITIAL_HIT_POINTS,
                            race: Race::Elf,
                        });
                        field
                    }
                    'G' => {
                        let field = Field::Npc(npcs.len());
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
        Self { map, npcs }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut npcs_per_line = Vec::new();
        for y in 0..self.map.height {
            npcs_per_line.clear();

            for x in 0..self.map.width {
                let field = &self.map[(x, y)];
                match field {
                    Field::Wall => f.write_char('#')?,
                    Field::Open => f.write_char('.')?,
                    Field::Npc(idx) => {
                        npcs_per_line.push(&self.npcs[*idx]);
                        match self.npcs[*idx].race {
                            Race::Elf => f.write_char('E')?,
                            Race::Goblin => f.write_char('G')?,
                        }
                    }
                };
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

pub fn solve(input: &str) -> (usize, usize) {
    let mut game = Game::from(input);
    let res = game.run();
    println!("{}", res.0 * res.1);
    res
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_game_do_turn() {
            const INPUT: &str = r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#;

            // initial
            let mut game = Game::from(INPUT);
            println!("{}", format!("{}", game));
            assert_eq!(
                format!("{}", game),
                r#"#######
#.G...#   G(200)
#...EG#   E(200), G(200)
#.#.#G#   G(200)
#..G#E#   G(200), E(200)
#.....#
#######
"#
            );

            // 1 round
            game.do_turn();
            println!("Got after turn: {}", format!("{}", game));

            assert_eq!(
                format!("{}", game),
                r#"#######
#..G..#   G(200)
#...EG#   E(197), G(197)
#.#G#G#   G(200), G(197)
#...#E#   E(197)
#.....#
#######
"#
            );

            // 2 rounds
            game.do_turn();
            println!("Got after turn: {}", format!("{}", game));

            assert_eq!(
                format!("{}", game),
                r#"#######
#...G.#   G(200)
#..GEG#   G(200), E(188), G(194)
#.#.#G#   G(194)
#...#E#   E(194)
#.....#
#######
"#
            );

            for _ in 2..23 {
                game.do_turn();
            }
            println!("Got after turn 23:\n{}", format!("{}", game));

            assert_eq!(
                format!("{}", game),
                r#"#######
#...G.#   G(200)
#..G.G#   G(200), G(131)
#.#.#G#   G(131)
#...#E#   E(131)
#.....#
#######
"#
            );

            game.do_turn();
            println!("Got after turn 24:\n{}", format!("{}", game));

            assert_eq!(
                format!("{}", game),
                r#"#######
#..G..#   G(200)
#...G.#   G(131)
#.#G#G#   G(200), G(128)
#...#E#   E(128)
#.....#
#######
"#
            );

            game.do_turn();
            println!("Got after turn 25:\n{}", format!("{}", game));
            assert_eq!(
                format!("{}", game),
                r#"#######
#.G...#   G(200)
#..G..#   G(131)
#.#.#G#   G(125)
#..G#E#   G(200), E(125)
#.....#
#######
"#
            );

            game.do_turn();
            println!("Got after turn 26:\n{}", format!("{}", game));
            assert_eq!(
                format!("{}", game),
                r#"#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(122)
#...#E#   E(122)
#..G..#   G(200)
#######
"#
            );

            game.do_turn();
            println!("Got after turn 27:\n{}", format!("{}", game));
            assert_eq!(
                format!("{}", game),
                r#"#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(119)
#...#E#   E(119)
#...G.#   G(200)
#######
"#
            );

            game.do_turn();
            println!("Got after turn 28:\n{}", format!("{}", game));
            assert_eq!(
                format!("{}", game),
                r#"#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(116)
#...#E#   E(113)
#....G#   G(200)
#######
"#
            );

            for _ in 28..47 {
                game.do_turn();
            }
            println!("Got after turn 47:\n{}", format!("{}", game));

            assert_eq!(
                format!("{}", game),
                r#"#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(59)
#...#.#
#....G#   G(200)
#######
"#
            );

            assert!(!game.do_turn());
        }

        #[test]
        fn test_run1() {
            let mut game = Game::from(
                r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#,
            );
            let score = game.run();
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#######
#G....#   G(200)
#.G...#   G(131)
#.#.#G#   G(59)
#...#.#
#....G#   G(200)
#######
"#
            );
            assert_eq!(score, (47, 590));
        }

        #[test]
        fn test_run2() {
            let mut game = Game::from(
                r#"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"#,
            );
            let score = game.run();
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#######
#...#E#   E(200)
#E#...#   E(197)
#.E##.#   E(185)
#E..#E#   E(200), E(200)
#.....#
#######
"#
            );
            assert_eq!(score, (37, 982));
        }

        #[test]
        fn test_run3() {
            let mut game = Game::from(
                r#"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"#,
            );
            let score = game.run();
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#######
#.E.E.#   E(164), E(197)
#.#E..#   E(200)
#E.##.#   E(98)
#.E.#.#   E(200)
#...#.#
#######
"#
            );
            assert_eq!(score, (46, 859));
        }

        #[test]
        fn test_run4() {
            let mut game = Game::from(
                r#"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"#,
            );
            let score = game.run();
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#######
#G.G#.#   G(200), G(98)
#.#G..#   G(200)
#..#..#
#...#G#   G(95)
#...G.#   G(200)
#######
"#,
            );
            assert_eq!(score, (35, 793));
        }

        #[test]
        fn test_run5() {
            let mut game = Game::from(
                r#"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"#,
            );
            let score = game.run();
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#######
#.....#
#.#G..#   G(200)
#.###.#
#.#.#.#
#G.G#G#   G(98), G(38), G(200)
#######
"#,
            );
            assert_eq!(score, (54, 536));
        }

        #[test]
        fn test_run6() {
            let mut game = Game::from(
                r#"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"#,
            );
            let score = game.run();
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#########
#.G.....#   G(137)
#G.G#...#   G(200), G(200)
#.G##...#   G(200)
#...##..#
#.G.#...#   G(200)
#.......#
#.......#
#########
"#,
            );
            assert_eq!(score, (20, 937));
        }

        #[test]
        fn test_move() {
            const INPUT: &str = r#"#######
#.E...#
#.....#
#...G.#
#######"#;
            let mut game = Game::from(INPUT);
            println!("{}", format!("{}", game));
            assert_eq!(
                format!("{}", game),
                r#"#######
#.E...#   E(200)
#.....#
#...G.#   G(200)
#######
"#
            );

            game.do_npc_move(0);
            assert_eq!(
                format!("{}", game),
                r#"#######
#..E..#   E(200)
#.....#
#...G.#   G(200)
#######
"#
            );

            game.do_npc_move(1);
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#######
#..E..#   E(200)
#...G.#   G(200)
#.....#
#######
"#
            );

            game.do_npc_move(0);
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#######
#...E.#   E(200)
#...G.#   G(200)
#.....#
#######
"#
            );
        }

        #[test]
        fn test_move2() {
            let mut game = Game::from(
                r#"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"#,
            );

            game.do_turn();
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#########
#.G...G.#   G(200), G(200)
#...G...#   G(197)
#...E..G#   E(200), G(200)
#.G.....#   G(200)
#.......#
#G..G..G#   G(200), G(200), G(200)
#.......#
#########
"#
            );

            game.do_turn();
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#########
#..G.G..#   G(200), G(200)
#...G...#   G(194)
#.G.E.G.#   G(200), E(197), G(200)
#.......#
#G..G..G#   G(200), G(200), G(200)
#.......#
#.......#
#########
"#
            );

            game.do_turn();
            println!("{}", game);
            assert_eq!(
                format!("{}", game),
                r#"#########
#.......#
#..GGG..#   G(200), G(191), G(200)
#..GEG..#   G(200), E(185), G(200)
#G..G...#   G(200), G(200)
#......G#   G(200)
#.......#
#.......#
#########
"#
            );
        }
    }
}
