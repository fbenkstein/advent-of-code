use std::fmt::{self, Write};
use std::mem;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

impl From<char> for Acre {
    fn from(c: char) -> Acre {
        match c {
            '.' => Acre::Open,
            '|' => Acre::Trees,
            '#' => Acre::Lumberyard,
            otherwise => panic!("unknown char: '{}'", otherwise),
        }
    }
}

impl Into<char> for Acre {
    fn into(self) -> char {
        match self {
            Acre::Open => '.',
            Acre::Trees => '|',
            Acre::Lumberyard => '#',
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Area(Vec<Acre>);

impl Area {
    fn dimension(&self) -> usize {
        (self.0.len() as f64).sqrt() as usize
    }

    fn next(&mut self) -> bool {
        let d = self.dimension() as isize;
        let mut next_area: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .map(|(idx, &acre)| {
                let i = idx as isize / d;
                let j = idx as isize % d;
                let adj_positions = [
                    (i - 1, j - 1),
                    (i, j - 1),
                    (i + 1, j - 1),
                    (i - 1, j),
                    (i + 1, j),
                    (i - 1, j + 1),
                    (i, j + 1),
                    (i + 1, j + 1),
                ];
                let adj = adj_positions.into_iter().filter_map(|&(i, j)| {
                    if 0 <= i && i < d && 0 <= j && j < d {
                        Some(self.0[(i * d + j) as usize])
                    } else {
                        None
                    }
                });
                let (num_trees, num_lumberyards) =
                    adj.fold((0, 0), |(num_trees, num_lumberyards), acre| match acre {
                        Acre::Trees => (num_trees + 1, num_lumberyards),
                        Acre::Lumberyard => (num_trees, num_lumberyards + 1),
                        _ => (num_trees, num_lumberyards),
                    });
                match acre {
                    Acre::Open => {
                        if num_trees >= 3 {
                            Acre::Trees
                        } else {
                            Acre::Open
                        }
                    }
                    Acre::Trees => {
                        if num_lumberyards >= 3 {
                            Acre::Lumberyard
                        } else {
                            Acre::Trees
                        }
                    }
                    Acre::Lumberyard => {
                        if num_lumberyards >= 1 && num_trees >= 1 {
                            Acre::Lumberyard
                        } else {
                            Acre::Open
                        }
                    }
                }
            })
            .collect();
        let is_changed = next_area != self.0;
        mem::swap(&mut next_area, &mut self.0);

        is_changed
    }

    fn resource_value(&self) -> usize {
        let num_trees = self.0.iter().filter(|&&a| a == Acre::Trees).count();
        let num_lumberyards = self.0.iter().filter(|&&a| a == Acre::Lumberyard).count();
        num_trees * num_lumberyards
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = self.dimension();
        for (i, &acre) in self.0.iter().enumerate() {
            f.write_char(acre.into())?;
            if i % d == d - 1 && i + 1 != self.0.len() {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

fn parse(input: &str) -> Area {
    let acres = input
        .chars()
        .filter(|&c| c != '\n')
        .map(Acre::from)
        .collect();
    Area(acres)
}

fn part1(mut area: Area) -> usize {
    for _ in 0..10 {
        area.next();
    }
    area.resource_value()
}

fn part2(mut area: Area) -> usize {
    let mut prev_generations = Vec::new();

    while area.next() {
        if prev_generations.iter().rev().any(|a| a == &area) {
            break;
        } else {
            prev_generations.push(area.clone());
        }
    }

    let cycle_start = prev_generations.iter().rposition(|a| a == &area).unwrap();
    let cycle_end = prev_generations.len();
    let cycle_len = cycle_end - cycle_start;

    // shift by 1 since we count from 0
    let end_pos = ((1_000_000_000 - 1 - cycle_start) % cycle_len) + cycle_start;

    prev_generations[end_pos].resource_value()
}

pub fn solve(input: &str) -> (usize, usize) {
    let area = parse(input);
    (part1(area.clone()), part2(area))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#;

    #[test]
    fn test_parse() {
        let a = parse(INPUT);
        let s = format!("{}", a);
        println!("{}", s);
        assert_eq!(a.dimension(), 10);
        assert_eq!(s, INPUT);
    }

    #[test]
    fn test_next() {
        let mut a = parse(INPUT);
        let s = format!("{}", a);
        println!("{}", s);

        for _ in 0..10 {
            a.next();
            println!("\n{}", a);
        }

        assert_eq!(
            format!("{}", a),
            r#".||##.....
||###.....
||##......
|##.....##
|##.....##
|##....##|
||##.####|
||#####|||
||||#|||||
||||||||||"#
        );
    }
}
