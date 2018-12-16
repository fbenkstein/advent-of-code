#[macro_use]
extern crate enumset;

use enumset::EnumSet;
use std::collections::{BTreeSet, VecDeque};
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
struct Cave {
    data: Vec<Field>,
    width: usize,
}

fn parse(input: &Vec<String>, att_elves: usize) -> Cave {
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
                    att: att_elves,
                    hp: 200,
                }),
                _ => panic!("Cave kaputt"),
            })
        }
    }
    Cave {
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

impl Cave {
    fn p(&self, pos: P) -> usize {
        pos.x + pos.y * self.width
    }

    fn field(&self, pos: P) -> &Field {
        &self.data[self.p(pos)]
    }

    fn field_mut(&mut self, pos: P) -> &mut Field {
        let index = self.p(pos);
        &mut self.data[index]
    }

    fn done(&self) -> bool {
        let combine = |mut sum, x: &Field| {
            if let Field::Unit(x) = x {
                sum |= x.class;
            }
            sum
        };
        self.data.iter().fold(EnumSet::new(), combine).len() == 1
    }

    fn units<'a>(&'a self) -> impl Iterator<Item = P> + 'a {
        let positions = move |x: &Field| match x {
            Field::Unit(unit) => Some(unit.pos),
            _ => None,
        };
        self.data.iter().filter_map(positions)
    }

    fn is_empty(&self, pos: P) -> bool {
        match self.field(pos) {
            Field::Empty => true,
            _ => false,
        }
    }

    fn attack_pos<'a>(&'a self, class: Class) -> impl Iterator<Item = P> + 'a {
        let others = move |x: &Field| match x {
            Field::Unit(other) if other.class != class => Some(other.pos),
            _ => None,
        };
        let empty = move |x: &P| self.is_empty(*x);
        let result = self.data.iter().filter_map(others);
        result.flat_map(P::neighbours).filter(empty)
    }

    fn step(&self, unit: &Unit) -> P {
        if let Some(_) = self.adjacent_enemy(unit) {
            return unit.pos;
        }

        let mut dist = vec![None; self.data.len()];
        let mut q: VecDeque<_> = self.attack_pos(unit.class).collect();
        for x in &q {
            dist[self.p(*x)] = Some(0);
        }
        while let Some(next) = q.pop_front() {
            for target in next.neighbours().filter(|pos| self.is_empty(*pos)) {
                if let None = dist[self.p(target)] {
                    dist[self.p(target)] = Some(dist[self.p(next)].unwrap() + 1);
                    q.push_back(target);
                }
            }
        }
        let reachable = |pos: P| dist[self.p(pos)].map(|d| (d, pos));
        let candidates = unit.pos.neighbours().filter_map(reachable);
        candidates.min().unwrap_or((0, unit.pos)).1
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

fn solve(mut input: Cave, print: bool) -> usize {
    let mut positions = BTreeSet::new();
    let mut iter = 0;
    if print {
        println!("Iteration {}", iter);
        input.print();
    }
    let mut full_round = true;
    let mut num_elves_died = 0;
    while !input.done() {
        for x in input.units() {
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
                    if let Class::Elf = enemy.class {
                        num_elves_died += 1;
                    }
                } else {
                    *input.field_mut(enemy_pos) = Field::Unit(enemy);
                }
            }

            *input.field_mut(new_pos) = Field::Unit(unit);

            if !positions.is_empty() && input.done() {
                full_round = false;
            }
        }
        if full_round {
            iter += 1;
        }

        if print {
            println!("");
            println!("Iteration {}", iter);
            input.print();
        }
    }

    let score: usize = input
        .units()
        .map(|pos| match input.field(pos) {
            Field::Unit(x) => x.hp,
            _ => 0,
        })
        .sum();

    println!("Done after {} iterations with {} hp left", iter, score);
    println!("Checksum: {}", score * iter);

    num_elves_died
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let mut attack_power = 3;
    loop {
        println!("Try attack power {}", attack_power);
        let input = parse(&lines, attack_power);
        if solve(input, false) == 0 {
            break;
        }
        attack_power += 1;
    }
}
