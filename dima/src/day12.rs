use std::fmt::{self, Write};
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Default, Clone)]
struct Field {
    lhs: Vec<bool>,
    rhs: Vec<bool>,
}

impl Field {
    fn with_positive(rhs: Vec<bool>) -> Self {
        Self {
            lhs: Vec::new(),
            rhs: rhs,
        }
    }

    fn start(&self) -> isize {
        -(self.lhs.len() as isize)
    }

    fn end(&self) -> isize {
        self.rhs.len() as isize
    }

    fn clear(&mut self) {
        self.lhs.clear();
        self.rhs.clear();
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = bool> + 'a {
        self.lhs
            .iter()
            .rev()
            .cloned()
            .chain(self.rhs.iter().cloned())
    }

    fn sum(&self) -> isize {
        self.iter()
            .enumerate()
            .map(|(idx, is_alive)| (idx as isize + self.start()) * (is_alive as isize))
            .sum()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &state in self.lhs.iter().rev().chain(self.rhs.iter()) {
            let c = if state == true { '#' } else { '.' };
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl Index<isize> for Field {
    type Output = bool;
    fn index(&self, idx: isize) -> &Self::Output {
        let (idx, side) = if idx < 0 {
            ((-idx - 1) as usize, &self.lhs)
        } else {
            (idx as usize, &self.rhs)
        };
        if idx >= side.len() {
            &false
        } else {
            &side[idx]
        }
    }
}

impl IndexMut<isize> for Field {
    fn index_mut(&mut self, idx: isize) -> &mut Self::Output {
        let (idx, side) = if idx < 0 {
            ((-idx - 1) as usize, &mut self.lhs)
        } else {
            (idx as usize, &mut self.rhs)
        };
        if idx >= side.len() {
            side.resize(idx + 1, false);
        }
        &mut side[idx]
    }
}

#[derive(Debug, Default)]
struct Rule {
    pattern: [bool; 5],
    // is_alive: bool,
}

impl Rule {
    fn parse(input: &str) -> Self {
        let mut rule = Self::default();
        let pattern = input[0..=5].chars().map(|c| c == '#');
        for (bit, val) in rule.pattern.iter_mut().zip(pattern) {
            *bit = val;
        }
        // rule.is_alive = input.chars().nth(9) == Some('#');
        rule
    }
}

fn parse(input: &str) -> (Field, Vec<Rule>) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let initial_generation = first_line.chars().skip(15).map(|c| c == '#').collect();
    let field = Field::with_positive(initial_generation);
    let rules = lines
        .skip(1)
        .filter(|p| p.chars().nth(9) == Some('#'))
        .map(Rule::parse)
        .collect();
    (field, rules)
}

fn solve1(mut gen: Field, rules: &[Rule]) -> isize {
    let mut next_gen = Field::default();

    for _ in 0..20 {
        next_gen.clear();
        (gen.start() - 2..gen.end() + 2)
            .filter(|&idx| {
                rules.iter().any(|rule| {
                    rule.pattern[0] == gen[idx - 2]
                        && rule.pattern[1] == gen[idx - 1]
                        && rule.pattern[2] == gen[idx]
                        && rule.pattern[3] == gen[idx + 1]
                        && rule.pattern[4] == gen[idx + 2]
                })
            })
            .for_each(|idx| next_gen[idx] = true);
        mem::swap(&mut gen, &mut next_gen);
    }

    gen.sum()
}

fn solve2(mut gen: Field, rules: &[Rule]) -> isize {
    let mut next_gen = Field::default();
    let mut prev_sum = 0;
    let mut prev_delta = 0;
    let mut stability = 0;

    for i in 0.. {
        next_gen.clear();
        (gen.start() - 2..gen.end() + 2)
            .filter(|&idx| {
                rules.iter().any(|rule| {
                    rule.pattern[0] == gen[idx - 2]
                        && rule.pattern[1] == gen[idx - 1]
                        && rule.pattern[2] == gen[idx]
                        && rule.pattern[3] == gen[idx + 1]
                        && rule.pattern[4] == gen[idx + 2]
                })
            })
            .for_each(|idx| next_gen[idx] = true);
        mem::swap(&mut gen, &mut next_gen);

        let sum = gen.sum();
        let delta = sum - prev_sum;
        if delta == prev_delta {
            if stability == 100 {
                return (50000000000 - i) * delta + prev_sum;
            } else {
                stability += 1;
            }
        } else {
            stability = 0;
        }
        prev_sum = sum;
        prev_delta = delta;
    }
    panic!("should not land here");
}

pub fn solve(input: &str) -> (isize, isize) {
    let (gen, rules) = parse(input);
    (solve1(gen.clone(), &rules), solve2(gen, &rules))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"#;

    #[test]
    fn test_parse() {
        let (field, rules) = parse(INPUT);
        assert_eq!(format!("{}", field), "#..#.#..##......###...###");
        assert_eq!(rules[0].pattern, [false, false, false, true, true]);
        assert_eq!(rules[1].pattern, [false, false, true, false, false]);
        assert_eq!(
            rules.last().unwrap().pattern,
            [true, true, true, true, false]
        );
    }
}
