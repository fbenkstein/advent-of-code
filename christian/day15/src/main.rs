#[macro_use]
extern crate enumset;

use enumset::EnumSet;
use std::collections::BTreeSet;
use std::io::{self, prelude::*};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy)]
struct P {
    y: usize,
    x: usize,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Field {
    Empty,
    Wall,
    Unit(Unit),
}

#[derive(Debug, EnumSetType)]
enum Class {
    Goblin,
    Elf,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Unit {
    pos: P,
    class: Class,
    att: usize,
    hp: usize,
}

#[derive(Debug)]
struct Input {
    data: Vec<Field>,
    width: usize,
}

fn parse(input: &Vec<String>) -> Input {
    let mut data = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            data.push(match c {
                b'#' => Field::Wall,
                b'.' => Field::Empty,
                b'G' => Field::Unit(Unit {
                    pos: P { y, x },
                    class: Class::Goblin,
                    att: 3,
                    hp: 200,
                }),
                b'E' => Field::Unit(Unit {
                    pos: P { y, x },
                    class: Class::Elf,
                    att: 3,
                    hp: 200,
                }),
                _ => panic!("Input kaputt"),
            })
        }
    }
    Input {
        data,
        width: input[0].bytes().len(),
    }
}

impl P {
    fn neighbours(self) -> impl Iterator<Item = P> {
        (0..4).map(move |i| P {
            y: (self.y as isize + [0, 0, 1, -1][i]) as usize,
            x: (self.x as isize + [1, -1, 0, 0][i]) as usize,
        })
    }
}

impl Input {
    fn to_pos(&self, pos: P) -> usize {
        pos.x + pos.y * self.width
    }

    fn field(&self, pos: P) -> &Field {
        &self.data[self.to_pos(pos)]
    }

    fn field_mut(&mut self, pos: P) -> &mut Field {
        let index = self.to_pos(pos);
        &mut self.data[index]
    }

    fn done(&self) -> bool {
        let sum = self.data.iter().fold(EnumSet::new(), |mut sum, x| {
            match x {
                Field::Unit(x) => sum |= x.class,
                _ => (),
            }
            sum
        });
        sum.len() == 1
    }

    fn order<'a>(&'a self) -> impl Iterator<Item = P> + 'a {
        let positions = move |(pos, x): (usize, &Field)| match x {
            Field::Unit(_) => Some(P {
                y: pos / self.width,
                x: pos % self.width,
            }),
            _ => None,
        };
        self.data.iter().enumerate().filter_map(positions)
    }

    fn step(&self, unit: &Unit) -> P {
        const INF: isize = std::isize::MAX;
        let mut dist = vec![INF; self.data.len()];
        dist[self.to_pos(unit.pos)] = 0;
        let mut q = vec![unit.pos];
        let mut q_pos = 0;
        let mut best = INF;
        let mut candidates = Vec::new();
        while q_pos < q.len() {
            let next = q[q_pos];
            let next_dist = dist[self.to_pos(next)];
            if next_dist > best {
                break;
            }
            for target in next.neighbours() {
                match &self.data[self.to_pos(target)] {
                    Field::Empty if dist[self.to_pos(target)] > next_dist + 1 => {
                        dist[self.to_pos(target)] = next_dist + 1;
                        q.push(target);
                    }
                    Field::Unit(other) if unit.class != other.class => {
                        candidates.push(next);
                        best = next_dist;
                    }
                    _ => (),
                }
            }

            q_pos += 1;
        }
        let best_pos = *candidates.iter().min().unwrap_or(&unit.pos);
        // println!("{:?} wants to go {:?}", unit, best_pos);

        // how to get there?
        q.clear();
        q_pos = 0;
        candidates.clear();
        q.push(best_pos);
        dist[self.to_pos(best_pos)] *= -1;
        while q_pos < q.len() {
            let next = q[q_pos];
            let next_dist = -dist[self.to_pos(next)];
            if next_dist == 1 {
                candidates.push(next);
                q_pos += 1;
                continue;
            }
            for target in next.neighbours() {
                if dist[self.to_pos(target)] == next_dist - 1 {
                    dist[self.to_pos(target)] *= -1;
                    q.push(target);
                }
            }
            q_pos += 1;
        }

        let result = *candidates.iter().min().unwrap_or(&unit.pos);
        // println!("{:?} steps to {:?}", unit, result);
        result
    }

    fn adjacent_enemy(&self, unit: &Unit) -> Option<Unit> {
        let is_enemy = |p: P| match self.field(p) {
            Field::Unit(x) if x.class != unit.class => Some(x.clone()),
            _ => None,
        };
        let hp = |x: &Unit| (x.hp, x.pos);
        unit.pos.neighbours().filter_map(is_enemy).min_by_key(hp)
    }

    fn print(&self) {
        let mut hps: Vec<usize> = Vec::new();
        for (i, x) in self.data.iter().enumerate() {
            let out = match x {
                Field::Wall => '#',
                Field::Empty => '.',
                Field::Unit(x) => {
                    hps.push(x.hp);
                    match x.class {
                        Class::Goblin => 'G',
                        Class::Elf => 'E',
                    }
                }
            };
            print!("{}", out);
            if (i + 1) % self.width == 0 {
                print!("    ");
                for hp in &hps {
                    print!(" ({})", hp);
                }
                println!("");
                hps.clear();
            }
        }
    }
}

fn solve(mut input: Input) {
    let mut positions = BTreeSet::new();
    let mut iter = 0;
    println!("Iteration {}", iter);
    input.print();
    let mut full_round = true;
    while !input.done() {
        for x in input.order() {
            positions.insert(x);
        }

        loop {
            let pos = match positions.iter().next() {
                Some(pos) => *pos,
                None => break,
            };
            positions.remove(&pos);
            let mut unit = match &input.field(pos) {
                Field::Unit(x) => (*x).clone(),
                _ => panic!("Corrupted data!"),
            };
            *input.field_mut(pos) = Field::Empty;

            let new_pos = input.step(&unit);
            unit.pos = new_pos;
            if let Some(mut enemy) = input.adjacent_enemy(&unit) {
                let enemy_pos = enemy.pos;
                enemy.hp = enemy.hp.saturating_sub(unit.att);
                if enemy.hp == 0 {
                    positions.remove(&enemy_pos);
                    *input.field_mut(enemy_pos) = Field::Empty;
                } else {
                    *input.field_mut(enemy_pos) = Field::Unit(enemy);
                }
            }

            *input.field_mut(new_pos) = Field::Unit(unit);

            if !positions.is_empty() && input.done() {
                full_round = false;
            }
        }
        println!("");
        if full_round {
            iter += 1;
        }

        println!("Iteration {}", iter);
        input.print();
    }

    let score: usize = input
        .order()
        .map(|pos| match input.field(pos) {
            Field::Unit(x) => x.hp,
            _ => 0,
        })
        .sum();

    println!("Done after {} iterations with {} hp left", iter, score);
    println!("Checksum: {}", score * iter);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    solve(input);
}
