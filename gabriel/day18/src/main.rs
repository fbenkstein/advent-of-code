use std::fmt::{self, Write};
use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;

#[derive(Clone, PartialEq, Eq)]
struct Area {
    collection: Vec<Acre>,
    size: isize,
}

impl From<&str> for Area {
    fn from(input: &str) -> Self {
        let size = input.lines().map(|l| l.len()).max().unwrap();
        let collection = input
            .lines()
            .flat_map(|line| -> Vec<Acre> { line.chars().map(|c| Acre::from(c)).collect() })
            .collect();
        Self {
            size: size as isize,
            collection,
        }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                let c = &self[(x, y)];
                f.write_char(c.into())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<(isize, isize)> for Area {
    type Output = Acre;
    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        if x > 0 || y > 0 || x < self.size || y < self.size {
            &self.collection[(x + y * self.size) as usize]
        } else {
            &Acre::Void
        }
    }
}

impl Area {
    fn adjacent_acres(&self, (x, y): (isize, isize)) -> [(isize, isize); 8] {
        [
            (x, y - 1),
            (x - 1, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x, y + 1),
            (x - 1, y + 1),
            (x + 1, y + 1),
        ]
    }

    fn next_state(&self, (x, y): (isize, isize)) -> Acre {
        let adjacent_acres = self
            .adjacent_acres((x, y))
            .into_iter()
            .filter(|&(x, y)| x >= &0 && x < &self.size && y >= &0 && y < &self.size)
            .map(|pos| self[*pos].clone())
            .collect::<Vec<_>>();
        match self[(x, y)] {
            Acre::OpenGround => {
                if adjacent_acres
                    .into_iter()
                    .filter(|a| a == &Acre::Trees)
                    .count()
                    >= 3
                {
                    Acre::Trees
                } else {
                    Acre::OpenGround
                }
            }
            Acre::Trees => {
                if adjacent_acres
                    .into_iter()
                    .filter(|a| a == &Acre::Lumberyard)
                    .count()
                    >= 3
                {
                    Acre::Lumberyard
                } else {
                    Acre::Trees
                }
            }
            Acre::Lumberyard => {
                if adjacent_acres.iter().any(|a| a == &Acre::Lumberyard)
                    && adjacent_acres.into_iter().any(|a| a == Acre::Trees)
                {
                    Acre::Lumberyard
                } else {
                    Acre::OpenGround
                }
            }
            Acre::Void => {
                panic!("Nooooooo!");
            }
        }
    }

    pub fn next(&mut self) -> Area {
        let mut buffer = Vec::with_capacity(self.size as usize ^ 2);
        for y in 0..self.size {
            for x in 0..self.size {
                buffer.push(self.next_state((x, y)));
            }
        }
        Self {
            collection: buffer,
            size: self.size,
        }
    }

    pub fn solve1(&self, max_time: usize) -> usize {
        let mut area = self.clone();
        for _ in 0..max_time {
            area = area.next();
        }
        area.value()
    }

    fn value(&self) -> usize {
        let wooded_acres = self
            .collection
            .iter()
            .filter(|&a| a == &Acre::Trees)
            .count();
        let lumberyards_acres = self
            .collection
            .iter()
            .filter(|&a| a == &Acre::Lumberyard)
            .count();
        wooded_acres * lumberyards_acres
    }

    /// see: https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_Tortoise_and_Hare
    pub fn solve2(&self) -> usize {
        let mut tortoise = self.clone().next();
        let mut hare = tortoise.clone().next();
        while tortoise != hare {
            tortoise = tortoise.next();
            hare = hare.next().next();
        }

        let mut mu = 0;
        let mut tortoise = self.clone();
        while tortoise != hare {
            tortoise = tortoise.next();
            hare = hare.next();
            mu += 1;
        }

        let mut lam = 1;
        let mut hare = tortoise.next();
        while tortoise != hare {
            hare = hare.next();
            lam += 1;
        }

        let mut x0 = self.clone();
        for _ in 0..1000000000 % lam + mu {
            x0 = x0.next();
        }

        x0.value()
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Acre {
    OpenGround,
    Trees,
    Lumberyard,
    Void,
}

impl From<char> for Acre {
    fn from(c: char) -> Self {
        match c {
            '.' => Acre::OpenGround,
            '|' => Acre::Trees,
            '#' => Acre::Lumberyard,
            _ => panic!("Unknown acre type!"),
        }
    }
}

impl Into<char> for &Acre {
    fn into(self) -> char {
        match self {
            Acre::OpenGround => '.',
            Acre::Trees => '|',
            Acre::Lumberyard => '#',
            Acre::Void => ' ',
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let area = Area::from(&contents[..]);
    println!(
        "Total resource value of the lumber collection area after 10 minutes: {}",
        area.solve1(10)
    );

    let area = Area::from(&contents[..]);
    println!("...: {}", area.solve2());
}

#[test]
fn test_example() {
    let input = r#".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
"#;

    let mut area = Area::from(input);
    assert_eq!(format!("{}", area), input);

    let end_input = r#".||##.....
||###.....
||##......
|##.....##
|##.....##
|##....##|
||##.####|
||#####|||
||||#|||||
||||||||||
"#;

    for _ in 0..10 {
        area.next();
        println!("{}", area);
    }

    assert_eq!(format!("{}", area), end_input);
}
